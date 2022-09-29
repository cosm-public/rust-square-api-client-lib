//! Model for Language enum

use serde::{Deserialize, Serialize};

/// A language in [BCP 47 format](https://tools.ietf.org/html/bcp47#appendix-A). For more
/// information, see [Location language
/// code](https://developer.squareup.com/docs/locations-api#location-language-code).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Language {
    /// English
    #[serde(rename = "en")]
    En,
    /// English - Australia
    #[serde(rename = "en-AU")]
    EnAu,
    /// English - Canada
    #[serde(rename = "en-CA")]
    EnCa,
    /// English - United Kingdom
    #[serde(rename = "en-GB")]
    EnGb,
    /// English - Ireland
    #[serde(rename = "en-IE")]
    EnIe,
    /// English - United States
    #[serde(rename = "en-US")]
    EnUs,
    /// Spanish
    #[serde(rename = "es")]
    Es,
    /// Spanish - United States
    #[serde(rename = "es-US")]
    EsUs,
    /// French
    #[serde(rename = "fr")]
    Fr,
    /// French - Canada
    #[serde(rename = "fr-CA")]
    FrCa,
    /// Japanese
    #[serde(rename = "ja")]
    Ja,
}
