//! UK HM Treasury OFSI consolidated sanctions list worker.
//!
//! Source: <https://assets.publishing.service.gov.uk/media/sanctions-consolidated-list-csv>
//! (the CSV publication of OFSI's consolidated list — the canonical
//! machine-readable form). The exact path is rotated by gov.uk; the URL
//! is configurable via [`UkOfsiWorker::with_url`].
//!
//! Format: CSV. Column ordering historically stable but field names
//! occasionally change capitalisation; we look up by **header name**, not
//! position, to survive that.

use async_trait::async_trait;
use std::collections::HashMap;

use crate::worker::{FeedError, FeedFetchResult, FeedWorker, RawFeed, SanctionEntry};

pub const UK_OFSI_CSV_URL: &str =
    "https://assets.publishing.service.gov.uk/media/sanctions-consolidated-list-csv";

pub struct UkOfsiWorker {
    client: reqwest::Client,
    url: String,
}

impl UkOfsiWorker {
    pub fn new() -> Self {
        Self {
            client: build_client(),
            url: UK_OFSI_CSV_URL.into(),
        }
    }

    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            client: build_client(),
            url: url.into(),
        }
    }
}

impl Default for UkOfsiWorker {
    fn default() -> Self {
        Self::new()
    }
}

fn build_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("CylinderSeal-FeedWorker/0.1")
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .expect("reqwest client")
}

#[async_trait]
impl FeedWorker for UkOfsiWorker {
    fn name(&self) -> &'static str {
        "UK_OFSI"
    }

    fn source_url(&self) -> &'static str {
        UK_OFSI_CSV_URL
    }

    async fn fetch(&self) -> Result<FeedFetchResult, FeedError> {
        let resp = self
            .client
            .get(&self.url)
            .send()
            .await
            .map_err(|e| FeedError::Network(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(FeedError::Network(format!("status {}", resp.status())));
        }
        let body = resp
            .bytes()
            .await
            .map_err(|e| FeedError::Network(e.to_string()))?
            .to_vec();
        let entries = parse_csv(&body)?;
        Ok(FeedFetchResult {
            raw: RawFeed {
                source_url: self.url.clone(),
                body,
            },
            entries,
        })
    }
}

/// Lower-cased header lookup so the parser tolerates HMT's occasional
/// "Group ID" / "GROUP ID" / "group_id" inconsistencies.
fn header_index(headers: &csv::StringRecord) -> HashMap<String, usize> {
    headers
        .iter()
        .enumerate()
        .map(|(i, h)| (normalise(h), i))
        .collect()
}

fn normalise(s: &str) -> String {
    s.trim().to_lowercase().replace([' ', '_', '-'], "")
}

fn parse_csv(bytes: &[u8]) -> Result<Vec<SanctionEntry>, FeedError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(bytes);
    let headers = rdr
        .headers()
        .map_err(|e| FeedError::Parse(format!("uk csv headers: {e}")))?
        .clone();
    let idx = header_index(&headers);
    let group_id = idx.get("groupid").copied();
    let name1 = idx.get("name1").or_else(|| idx.get("name")).copied();
    let name2 = idx.get("name2").copied();
    let name3 = idx.get("name3").copied();
    let name6 = idx.get("name6").copied();
    let alias = idx.get("aliastype").copied(); // "Primary Name" vs "AKA"
    let country = idx
        .get("country")
        .or_else(|| idx.get("countryofbirth"))
        .copied();
    let regime = idx
        .get("regimename")
        .or_else(|| idx.get("regime"))
        .copied();
    let group_type = idx
        .get("grouptype")
        .or_else(|| idx.get("type"))
        .copied();

    // Group rows by group_id so we collapse the per-alias rows that HMT
    // emits as separate lines into a single SanctionEntry.
    let mut groups: HashMap<String, GroupAccum> = HashMap::new();
    let mut order: Vec<String> = Vec::new();

    for (i, rec) in rdr.records().enumerate() {
        let r = rec.map_err(|e| FeedError::Parse(format!("uk row {i}: {e}")))?;
        let id = group_id
            .and_then(|j| r.get(j))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("row{i}"));
        let full_name = join_nonempty(&[
            name1.and_then(|j| r.get(j)),
            name2.and_then(|j| r.get(j)),
            name3.and_then(|j| r.get(j)),
            name6.and_then(|j| r.get(j)),
        ]);
        if full_name.is_empty() {
            continue;
        }
        let is_primary = alias
            .and_then(|j| r.get(j))
            .map(|s| s.trim().eq_ignore_ascii_case("Primary Name"))
            .unwrap_or(true);

        let acc = groups.entry(id.clone()).or_insert_with(|| {
            order.push(id.clone());
            GroupAccum {
                primary: String::new(),
                aliases: Vec::new(),
                country: None,
                program: None,
                entity_type: "individual".into(),
            }
        });
        if is_primary && acc.primary.is_empty() {
            acc.primary = full_name.clone();
        } else if full_name != acc.primary {
            acc.aliases.push(full_name);
        }
        if acc.country.is_none() {
            acc.country = country
                .and_then(|j| r.get(j))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());
        }
        if acc.program.is_none() {
            acc.program = regime
                .and_then(|j| r.get(j))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());
        }
        if let Some(j) = group_type {
            if let Some(t) = r.get(j) {
                let t = t.trim().to_lowercase();
                if t.contains("entity") || t.contains("organisation") {
                    acc.entity_type = "entity".into();
                }
            }
        }
    }

    let out = order
        .into_iter()
        .filter_map(|id| {
            groups.remove(&id).and_then(|g| {
                if g.primary.is_empty() {
                    None
                } else {
                    let mut aliases = g.aliases;
                    aliases.sort();
                    aliases.dedup();
                    Some(SanctionEntry {
                        source: "UK_OFSI".into(),
                        external_id: id.clone(),
                        primary_name: g.primary,
                        aliases,
                        entity_type: g.entity_type,
                        country: g.country,
                        program: g.program,
                        raw: serde_json::json!({"groupId": id}),
                    })
                }
            })
        })
        .collect();
    Ok(out)
}

struct GroupAccum {
    primary: String,
    aliases: Vec<String>,
    country: Option<String>,
    program: Option<String>,
    entity_type: String,
}

fn join_nonempty(parts: &[Option<&str>]) -> String {
    parts
        .iter()
        .filter_map(|p| p.map(str::trim).filter(|s| !s.is_empty()))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_alias_rows_into_one_entry() {
        let csv = b"Group ID,Name 1,Name 2,Alias Type,Country,Regime Name,Group Type\n\
1001,John,Smith,Primary Name,UK,RUSSIA,Individual\n\
1001,Johnny,Smith,AKA,,RUSSIA,Individual\n\
1002,Acme,,Primary Name,GB,RUSSIA,Entity\n";
        let entries = parse_csv(csv).expect("parse");
        assert_eq!(entries.len(), 2);
        let john = entries.iter().find(|e| e.external_id == "1001").unwrap();
        assert_eq!(john.primary_name, "John Smith");
        assert_eq!(john.aliases, vec!["Johnny Smith"]);
        assert_eq!(john.country.as_deref(), Some("UK"));
        assert_eq!(john.program.as_deref(), Some("RUSSIA"));
        assert_eq!(john.entity_type, "individual");
        let acme = entries.iter().find(|e| e.external_id == "1002").unwrap();
        assert_eq!(acme.primary_name, "Acme");
        assert_eq!(acme.entity_type, "entity");
    }

    #[test]
    fn tolerates_alternative_header_casing() {
        let csv = b"GROUP ID,NAME 1,Alias Type,REGIME NAME\n\
77,Foo,Primary Name,LIBYA\n";
        let entries = parse_csv(csv).expect("parse");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].external_id, "77");
        assert_eq!(entries[0].primary_name, "Foo");
        assert_eq!(entries[0].program.as_deref(), Some("LIBYA"));
    }
}
