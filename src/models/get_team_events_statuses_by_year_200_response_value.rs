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
pub struct GetTeamEventsStatusesByYear200ResponseValue {
    #[serde(rename = "qual", skip_serializing_if = "Option::is_none")]
    pub qual: Option<Box<models::TeamEventStatusRank>>,
    #[serde(rename = "alliance", skip_serializing_if = "Option::is_none")]
    pub alliance: Option<Box<models::TeamEventStatusAlliance>>,
    #[serde(rename = "playoff", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub playoff: Option<Option<Box<models::TeamEventStatusPlayoff>>>,
    /// An HTML formatted string suitable for display to the user containing the team's alliance pick status.
    #[serde(rename = "alliance_status_str", skip_serializing_if = "Option::is_none")]
    pub alliance_status_str: Option<String>,
    /// An HTML formatter string suitable for display to the user containing the team's playoff status.
    #[serde(rename = "playoff_status_str", skip_serializing_if = "Option::is_none")]
    pub playoff_status_str: Option<String>,
    /// An HTML formatted string suitable for display to the user containing the team's overall status summary of the event.
    #[serde(rename = "overall_status_str", skip_serializing_if = "Option::is_none")]
    pub overall_status_str: Option<String>,
    /// TBA match key for the next match the team is scheduled to play in at this event, or null.
    #[serde(rename = "next_match_key", skip_serializing_if = "Option::is_none")]
    pub next_match_key: Option<String>,
    /// TBA match key for the last match the team played in at this event, or null.
    #[serde(rename = "last_match_key", skip_serializing_if = "Option::is_none")]
    pub last_match_key: Option<String>,
}

impl GetTeamEventsStatusesByYear200ResponseValue {
    pub fn new() -> GetTeamEventsStatusesByYear200ResponseValue {
        GetTeamEventsStatusesByYear200ResponseValue {
            qual: None,
            alliance: None,
            playoff: None,
            alliance_status_str: None,
            playoff_status_str: None,
            overall_status_str: None,
            next_match_key: None,
            last_match_key: None,
        }
    }
}

