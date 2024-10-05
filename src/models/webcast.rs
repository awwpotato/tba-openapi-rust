/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.9.5
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webcast {
    /// Type of webcast, typically descriptive of the streaming provider.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Type specific channel information. May be the YouTube stream, or Twitch channel name. In the case of iframe types, contains HTML to embed the stream in an HTML iframe.
    #[serde(rename = "channel")]
    pub channel: String,
    /// The date for the webcast in `yyyy-mm-dd` format. May be null.
    #[serde(rename = "date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date: Option<Option<String>>,
    /// File identification as may be required for some types. May be null.
    #[serde(rename = "file", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub file: Option<Option<String>>,
}

impl Webcast {
    pub fn new(r#type: Type, channel: String) -> Webcast {
        Webcast {
            r#type,
            channel,
            date: None,
            file: None,
        }
    }
}
/// Type of webcast, typically descriptive of the streaming provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "youtube")]
    Youtube,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "ustream")]
    Ustream,
    #[serde(rename = "iframe")]
    Iframe,
    #[serde(rename = "html5")]
    Html5,
    #[serde(rename = "rtmp")]
    Rtmp,
    #[serde(rename = "livestream")]
    Livestream,
    #[serde(rename = "direct_link")]
    DirectLink,
    #[serde(rename = "mms")]
    Mms,
    #[serde(rename = "justin")]
    Justin,
    #[serde(rename = "stemtv")]
    Stemtv,
    #[serde(rename = "dacast")]
    Dacast,
}

impl Default for Type {
    fn default() -> Type {
        Self::Youtube
    }
}

