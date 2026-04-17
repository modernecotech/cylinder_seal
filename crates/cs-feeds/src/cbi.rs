//! Central Bank of Iraq (CBI) domestic sanctions list worker.
//!
//! CBI publishes its consolidated domestic sanctions list as a CSV
//! attachment under <https://cbi.iq>. The exact path changes; the URL
//! is therefore configurable via [`CbiSanctionsWorker::with_url`] and
//! the default is left as a placeholder pending CBI's publication of a
//! stable machine-readable endpoint.
//!
//! Format (assumed): CSV with headers
//! `id,name,aliases,entity_type,country,program`
//! where `aliases` is a `;`-separated string. If/when CBI publishes XML
//! instead, swap the `parse_csv` body for a `quick_xml` parser.

use async_trait::async_trait;

use crate::worker::{FeedError, FeedFetchResult, FeedWorker, RawFeed, SanctionEntry};

pub const CBI_DEFAULT_URL: &str = "https://cbi.iq/static/uploads/up/sanctions.csv";

pub struct CbiSanctionsWorker {
    client: reqwest::Client,
    url: String,
}

impl CbiSanctionsWorker {
    pub fn new() -> Self {
        Self {
            client: build_client(),
            url: CBI_DEFAULT_URL.into(),
        }
    }

    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            client: build_client(),
            url: url.into(),
        }
    }
}

impl Default for CbiSanctionsWorker {
    fn default() -> Self {
        Self::new()
    }
}

fn build_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("CylinderSeal-FeedWorker/0.1")
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("reqwest client")
}

#[async_trait]
impl FeedWorker for CbiSanctionsWorker {
    fn name(&self) -> &'static str {
        "CBI_IQ"
    }

    fn source_url(&self) -> &'static str {
        CBI_DEFAULT_URL
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

fn parse_csv(bytes: &[u8]) -> Result<Vec<SanctionEntry>, FeedError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(bytes);
    let mut out = Vec::new();
    for (i, rec) in rdr.records().enumerate() {
        let r = rec.map_err(|e| FeedError::Parse(format!("row {i}: {e}")))?;
        let id = r.get(0).unwrap_or("").trim().to_string();
        let name = r.get(1).unwrap_or("").trim().to_string();
        if id.is_empty() || name.is_empty() {
            continue;
        }
        let aliases: Vec<String> = r
            .get(2)
            .unwrap_or("")
            .split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let entity_type = r.get(3).unwrap_or("individual").trim().to_lowercase();
        let country = r.get(4).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let program = r.get(5).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        out.push(SanctionEntry {
            source: "CBI_IQ".into(),
            external_id: id.clone(),
            primary_name: name,
            aliases,
            entity_type,
            country,
            program,
            raw: serde_json::json!({"row": i, "id": id}),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_csv_skipping_blank_rows() {
        let csv_bytes = b"id,name,aliases,entity_type,country,program\n\
            1,Acme Holding,Acme;ACME LTD,entity,IQ,DOMESTIC\n\
            ,,,,,\n\
            2,Joe Q,JQ;Joey,individual,IQ,DOMESTIC\n";
        let entries = parse_csv(csv_bytes).expect("parse");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].external_id, "1");
        assert_eq!(entries[0].aliases, vec!["Acme", "ACME LTD"]);
        assert_eq!(entries[1].entity_type, "individual");
    }
}
