//! US OFAC SDN list worker.
//!
//! Source: <https://sanctionslistservice.ofac.treas.gov/api/PublicationPreview/exports/SDN.XML>
//! (a long-stable Treasury endpoint; the legacy `treasury.gov/ofac/downloads`
//! mirror is also acceptable).
//!
//! The XML schema is documented as the SDN Advanced format. We parse a
//! minimal subset — uid, sdnType, primary name, aliases, programs,
//! country — sufficient for blacklist match. Full sanctions screening
//! (DOB, address fuzzy match) is out of scope here; a downstream
//! cs-screening crate can take the same `SanctionEntry` rows.

use async_trait::async_trait;
use quick_xml::de::from_str;
use serde::Deserialize;

use crate::worker::{FeedError, FeedFetchResult, FeedWorker, RawFeed, SanctionEntry};

pub const OFAC_SDN_XML_URL: &str =
    "https://sanctionslistservice.ofac.treas.gov/api/PublicationPreview/exports/SDN.XML";

pub struct OfacSdnWorker {
    client: reqwest::Client,
    url: String,
}

impl OfacSdnWorker {
    pub fn new() -> Self {
        Self {
            client: build_client(),
            url: OFAC_SDN_XML_URL.into(),
        }
    }

    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            client: build_client(),
            url: url.into(),
        }
    }
}

impl Default for OfacSdnWorker {
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
impl FeedWorker for OfacSdnWorker {
    fn name(&self) -> &'static str {
        "OFAC_SDN"
    }

    fn source_url(&self) -> &'static str {
        OFAC_SDN_XML_URL
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
        let entries = parse_sdn(text)?;
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
#[serde(rename_all = "camelCase")]
struct SdnList {
    #[serde(rename = "sdnEntry", default)]
    sdn_entry: Vec<SdnEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SdnEntry {
    uid: String,
    #[serde(default)]
    first_name: Option<String>,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(rename = "sdnType", default)]
    sdn_type: Option<String>,
    #[serde(rename = "programList", default)]
    program_list: Option<ProgramList>,
    #[serde(rename = "akaList", default)]
    aka_list: Option<AkaList>,
    #[serde(rename = "addressList", default)]
    address_list: Option<AddressList>,
}

#[derive(Debug, Deserialize)]
struct ProgramList {
    #[serde(default)]
    program: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AkaList {
    #[serde(default)]
    aka: Vec<Aka>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Aka {
    #[serde(default)]
    first_name: Option<String>,
    #[serde(default)]
    last_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AddressList {
    #[serde(default)]
    address: Vec<Address>,
}

#[derive(Debug, Deserialize)]
struct Address {
    #[serde(default)]
    country: Option<String>,
}

fn parse_sdn(text: &str) -> Result<Vec<SanctionEntry>, FeedError> {
    let parsed: SdnList = from_str(text).map_err(|e| FeedError::Parse(e.to_string()))?;
    let mut out = Vec::with_capacity(parsed.sdn_entry.len());
    for e in parsed.sdn_entry {
        let primary = format!(
            "{} {}",
            e.first_name.clone().unwrap_or_default(),
            e.last_name.clone().unwrap_or_default()
        )
        .trim()
        .to_string();
        let aliases = e
            .aka_list
            .map(|al| {
                al.aka
                    .into_iter()
                    .map(|a| {
                        format!(
                            "{} {}",
                            a.first_name.unwrap_or_default(),
                            a.last_name.unwrap_or_default()
                        )
                        .trim()
                        .to_string()
                    })
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        let country = e
            .address_list
            .and_then(|al| al.address.into_iter().find_map(|a| a.country));
        let program = e
            .program_list
            .and_then(|pl| pl.program.into_iter().next());
        out.push(SanctionEntry {
            source: "OFAC_SDN".into(),
            external_id: e.uid.clone(),
            primary_name: primary,
            aliases,
            entity_type: e
                .sdn_type
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "individual".into()),
            country,
            program,
            raw: serde_json::json!({"uid": e.uid}),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_sdn_xml() {
        let xml = r#"
            <sdnList>
                <sdnEntry>
                    <uid>1001</uid>
                    <firstName>Mahmoud</firstName>
                    <lastName>Test</lastName>
                    <sdnType>Individual</sdnType>
                    <programList><program>SDGT</program></programList>
                    <akaList>
                        <aka><firstName>M.</firstName><lastName>Test</lastName></aka>
                    </akaList>
                    <addressList>
                        <address><country>Iran</country></address>
                    </addressList>
                </sdnEntry>
            </sdnList>
        "#;
        let entries = parse_sdn(xml).expect("parse");
        assert_eq!(entries.len(), 1);
        let e = &entries[0];
        assert_eq!(e.external_id, "1001");
        assert_eq!(e.primary_name, "Mahmoud Test");
        assert_eq!(e.aliases, vec!["M. Test".to_string()]);
        assert_eq!(e.country.as_deref(), Some("Iran"));
        assert_eq!(e.program.as_deref(), Some("SDGT"));
        assert_eq!(e.entity_type, "individual");
    }

    #[test]
    fn parses_empty_sdn_list() {
        let xml = r#"<sdnList></sdnList>"#;
        let entries = parse_sdn(xml).expect("parse");
        assert_eq!(entries.len(), 0);
    }
}
