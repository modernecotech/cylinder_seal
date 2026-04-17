//! EU Common Foreign and Security Policy (CFSP) consolidated sanctions
//! list worker.
//!
//! Source: <https://webgate.ec.europa.eu/fsd/fsf/public/files/jsonFullSanctionsList_1_1/content>
//! (the public JSON export maintained by the European External Action
//! Service / DG FISMA).
//!
//! The schema is large; we project only the fields needed for screening
//! into the canonical `SanctionEntry`. For richer fields (DOB, place of
//! birth, address) the `raw` field carries a JSON pointer back to the
//! original entity.

use async_trait::async_trait;
use serde::Deserialize;

use crate::worker::{FeedError, FeedFetchResult, FeedWorker, RawFeed, SanctionEntry};

pub const EU_CFSP_JSON_URL: &str =
    "https://webgate.ec.europa.eu/fsd/fsf/public/files/jsonFullSanctionsList_1_1/content";

pub struct EuCfspWorker {
    client: reqwest::Client,
    url: String,
}

impl EuCfspWorker {
    pub fn new() -> Self {
        Self {
            client: build_client(),
            url: EU_CFSP_JSON_URL.into(),
        }
    }

    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            client: build_client(),
            url: url.into(),
        }
    }
}

impl Default for EuCfspWorker {
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
impl FeedWorker for EuCfspWorker {
    fn name(&self) -> &'static str {
        "EU_CFSP"
    }

    fn source_url(&self) -> &'static str {
        EU_CFSP_JSON_URL
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
        let entries = parse_json(&body)?;
        Ok(FeedFetchResult {
            raw: RawFeed {
                source_url: self.url.clone(),
                body,
            },
            entries,
        })
    }
}

/// Top-level wrapper. The EU schema nests entities under `data.export`.
/// Our parser is permissive: missing optional fields default to empty.
#[derive(Debug, Deserialize)]
struct EuExport {
    #[serde(rename = "exportEntities", default)]
    entities: Vec<EuEntity>,
}

#[derive(Debug, Deserialize)]
struct EuEntity {
    #[serde(rename = "logicalId")]
    logical_id: i64,
    #[serde(rename = "subjectType", default)]
    subject_type: Option<EuSubjectType>,
    #[serde(rename = "nameAlias", default)]
    name_alias: Vec<EuNameAlias>,
    #[serde(rename = "regulation", default)]
    regulation: Option<EuRegulation>,
    #[serde(rename = "citizenship", default)]
    citizenship: Vec<EuCitizenship>,
}

#[derive(Debug, Deserialize)]
struct EuSubjectType {
    code: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EuNameAlias {
    #[serde(rename = "wholeName", default)]
    whole_name: Option<String>,
    #[serde(rename = "strong", default)]
    strong: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct EuRegulation {
    #[serde(rename = "programme", default)]
    programme: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EuCitizenship {
    #[serde(rename = "countryIso2Code", default)]
    country_iso2: Option<String>,
}

fn parse_json(bytes: &[u8]) -> Result<Vec<SanctionEntry>, FeedError> {
    let parsed: EuExport = serde_json::from_slice(bytes)
        .map_err(|e| FeedError::Parse(format!("eu json: {e}")))?;
    let mut out = Vec::with_capacity(parsed.entities.len());
    for e in parsed.entities {
        // Strong name = the canonical legal name; fall back to first
        // alias if no entry is marked strong.
        let primary = e
            .name_alias
            .iter()
            .find(|a| a.strong.unwrap_or(false))
            .or_else(|| e.name_alias.first())
            .and_then(|a| a.whole_name.clone())
            .unwrap_or_default();
        let aliases = e
            .name_alias
            .iter()
            .filter_map(|a| a.whole_name.clone())
            .filter(|s| s != &primary && !s.trim().is_empty())
            .collect();
        let entity_type = match e.subject_type.and_then(|s| s.code).as_deref() {
            Some("P") => "individual",
            Some("E") => "entity",
            _ => "individual",
        }
        .to_string();
        let country = e
            .citizenship
            .into_iter()
            .find_map(|c| c.country_iso2);
        let program = e.regulation.and_then(|r| r.programme);
        out.push(SanctionEntry {
            source: "EU_CFSP".into(),
            external_id: e.logical_id.to_string(),
            primary_name: primary,
            aliases,
            entity_type,
            country,
            program,
            raw: serde_json::json!({"logicalId": e.logical_id}),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_individual_with_strong_alias() {
        let json = br#"{
            "exportEntities": [
                {
                    "logicalId": 12345,
                    "subjectType": {"code": "P"},
                    "nameAlias": [
                        {"wholeName": "Some Person", "strong": true},
                        {"wholeName": "S. Person", "strong": false}
                    ],
                    "regulation": {"programme": "SYRIA"},
                    "citizenship": [{"countryIso2Code": "SY"}]
                }
            ]
        }"#;
        let entries = parse_json(json).expect("parse");
        assert_eq!(entries.len(), 1);
        let e = &entries[0];
        assert_eq!(e.external_id, "12345");
        assert_eq!(e.primary_name, "Some Person");
        assert_eq!(e.aliases, vec!["S. Person".to_string()]);
        assert_eq!(e.entity_type, "individual");
        assert_eq!(e.country.as_deref(), Some("SY"));
        assert_eq!(e.program.as_deref(), Some("SYRIA"));
    }

    #[test]
    fn parses_entity_falls_back_to_first_alias_when_no_strong() {
        let json = br#"{
            "exportEntities": [
                {
                    "logicalId": 99,
                    "subjectType": {"code": "E"},
                    "nameAlias": [
                        {"wholeName": "Acme Front Co", "strong": false}
                    ]
                }
            ]
        }"#;
        let entries = parse_json(json).expect("parse");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].primary_name, "Acme Front Co");
        assert_eq!(entries[0].entity_type, "entity");
        assert!(entries[0].aliases.is_empty());
        assert!(entries[0].country.is_none());
    }

    #[test]
    fn parses_empty_export() {
        let json = br#"{"exportEntities": []}"#;
        let entries = parse_json(json).expect("parse");
        assert!(entries.is_empty());
    }
}
