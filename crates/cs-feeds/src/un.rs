//! UN Security Council Consolidated Sanctions List worker.
//!
//! Source: <https://scsanctions.un.org/resources/xml/en/consolidated.xml>
//! (the canonical, regularly-updated list maintained by the UNSC).
//!
//! Parses individuals + entities into the same `SanctionEntry` shape as
//! OFAC so downstream screening logic is source-agnostic.

use async_trait::async_trait;
use quick_xml::de::from_str;
use serde::Deserialize;

use crate::worker::{FeedError, FeedFetchResult, FeedWorker, RawFeed, SanctionEntry};

pub const UN_CONSOLIDATED_XML_URL: &str =
    "https://scsanctions.un.org/resources/xml/en/consolidated.xml";

pub struct UnConsolidatedWorker {
    client: reqwest::Client,
    url: String,
}

impl UnConsolidatedWorker {
    pub fn new() -> Self {
        Self {
            client: build_client(),
            url: UN_CONSOLIDATED_XML_URL.into(),
        }
    }

    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            client: build_client(),
            url: url.into(),
        }
    }
}

impl Default for UnConsolidatedWorker {
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
impl FeedWorker for UnConsolidatedWorker {
    fn name(&self) -> &'static str {
        "UN_CONS"
    }

    fn source_url(&self) -> &'static str {
        UN_CONSOLIDATED_XML_URL
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
        let text = std::str::from_utf8(&body)
            .map_err(|e| FeedError::Parse(format!("utf8: {e}")))?;
        let entries = parse_consolidated(text)?;
        Ok(FeedFetchResult {
            raw: RawFeed {
                source_url: self.url.clone(),
                body,
            },
            entries,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "CONSOLIDATED_LIST")]
struct ConsolidatedList {
    #[serde(rename = "INDIVIDUALS", default)]
    individuals: Option<Individuals>,
    #[serde(rename = "ENTITIES", default)]
    entities: Option<Entities>,
}

#[derive(Debug, Deserialize)]
struct Individuals {
    #[serde(rename = "INDIVIDUAL", default)]
    individual: Vec<Individual>,
}

#[derive(Debug, Deserialize)]
struct Entities {
    #[serde(rename = "ENTITY", default)]
    entity: Vec<Entity>,
}

#[derive(Debug, Deserialize)]
struct Individual {
    #[serde(rename = "DATAID")]
    dataid: String,
    #[serde(rename = "FIRST_NAME", default)]
    first_name: Option<String>,
    #[serde(rename = "SECOND_NAME", default)]
    second_name: Option<String>,
    #[serde(rename = "THIRD_NAME", default)]
    third_name: Option<String>,
    #[serde(rename = "UN_LIST_TYPE", default)]
    list_type: Option<String>,
    #[serde(rename = "INDIVIDUAL_ALIAS", default)]
    aliases: Vec<IndividualAlias>,
    #[serde(rename = "NATIONALITY", default)]
    nationality: Option<Nationality>,
}

#[derive(Debug, Deserialize)]
struct IndividualAlias {
    #[serde(rename = "ALIAS_NAME", default)]
    alias_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Nationality {
    #[serde(rename = "VALUE", default)]
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Entity {
    #[serde(rename = "DATAID")]
    dataid: String,
    #[serde(rename = "FIRST_NAME", default)]
    first_name: Option<String>,
    #[serde(rename = "UN_LIST_TYPE", default)]
    list_type: Option<String>,
    #[serde(rename = "ENTITY_ALIAS", default)]
    aliases: Vec<EntityAlias>,
}

#[derive(Debug, Deserialize)]
struct EntityAlias {
    #[serde(rename = "ALIAS_NAME", default)]
    alias_name: Option<String>,
}

fn parse_consolidated(text: &str) -> Result<Vec<SanctionEntry>, FeedError> {
    let parsed: ConsolidatedList =
        from_str(text).map_err(|e| FeedError::Parse(e.to_string()))?;
    let mut out = Vec::new();
    if let Some(inds) = parsed.individuals {
        for i in inds.individual {
            let primary = [&i.first_name, &i.second_name, &i.third_name]
                .iter()
                .filter_map(|x| x.as_deref())
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            let aliases = i
                .aliases
                .into_iter()
                .filter_map(|a| a.alias_name)
                .filter(|s| !s.trim().is_empty())
                .collect();
            let country = i.nationality.and_then(|n| n.value);
            out.push(SanctionEntry {
                source: "UN_CONS".into(),
                external_id: i.dataid.clone(),
                primary_name: primary,
                aliases,
                entity_type: "individual".into(),
                country,
                program: i.list_type,
                raw: serde_json::json!({"dataid": i.dataid}),
            });
        }
    }
    if let Some(ents) = parsed.entities {
        for e in ents.entity {
            let aliases = e
                .aliases
                .into_iter()
                .filter_map(|a| a.alias_name)
                .filter(|s| !s.trim().is_empty())
                .collect();
            out.push(SanctionEntry {
                source: "UN_CONS".into(),
                external_id: e.dataid.clone(),
                primary_name: e.first_name.unwrap_or_default(),
                aliases,
                entity_type: "entity".into(),
                country: None,
                program: e.list_type,
                raw: serde_json::json!({"dataid": e.dataid}),
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_individual_and_entity() {
        let xml = r#"
            <CONSOLIDATED_LIST>
                <INDIVIDUALS>
                    <INDIVIDUAL>
                        <DATAID>QDi.001</DATAID>
                        <FIRST_NAME>John</FIRST_NAME>
                        <SECOND_NAME>Doe</SECOND_NAME>
                        <UN_LIST_TYPE>Al-Qaida</UN_LIST_TYPE>
                        <INDIVIDUAL_ALIAS>
                            <ALIAS_NAME>JD</ALIAS_NAME>
                        </INDIVIDUAL_ALIAS>
                        <NATIONALITY><VALUE>Yemen</VALUE></NATIONALITY>
                    </INDIVIDUAL>
                </INDIVIDUALS>
                <ENTITIES>
                    <ENTITY>
                        <DATAID>QDe.001</DATAID>
                        <FIRST_NAME>Some Org</FIRST_NAME>
                        <UN_LIST_TYPE>Al-Qaida</UN_LIST_TYPE>
                        <ENTITY_ALIAS><ALIAS_NAME>SO</ALIAS_NAME></ENTITY_ALIAS>
                    </ENTITY>
                </ENTITIES>
            </CONSOLIDATED_LIST>
        "#;
        let entries = parse_consolidated(xml).expect("parse");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].entity_type, "individual");
        assert_eq!(entries[0].primary_name, "John Doe");
        assert_eq!(entries[1].entity_type, "entity");
        assert_eq!(entries[1].primary_name, "Some Org");
    }
}
