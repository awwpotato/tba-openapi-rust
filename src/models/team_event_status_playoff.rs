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

/// TeamEventStatusPlayoff : Playoff status for this team, may be null if the team did not make playoffs, or playoffs have not begun.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamEventStatusPlayoff {
    /// The highest playoff level the team reached.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(rename = "current_level_record", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub current_level_record: Option<Option<Box<models::WltRecord>>>,
    #[serde(rename = "record", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub record: Option<Option<Box<models::WltRecord>>>,
    /// Current competition status for the playoffs.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The average match score during playoffs. Year specific. May be null if not relevant for a given year.
    #[serde(rename = "playoff_average", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub playoff_average: Option<Option<i32>>,
}

impl TeamEventStatusPlayoff {
    /// Playoff status for this team, may be null if the team did not make playoffs, or playoffs have not begun.
    pub fn new() -> TeamEventStatusPlayoff {
        TeamEventStatusPlayoff {
            level: None,
            current_level_record: None,
            record: None,
            status: None,
            playoff_average: None,
        }
    }
}
/// The highest playoff level the team reached.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "qm")]
    Qm,
    #[serde(rename = "ef")]
    Ef,
    #[serde(rename = "qf")]
    Qf,
    #[serde(rename = "sf")]
    Sf,
    #[serde(rename = "f")]
    F,
}

impl Default for Level {
    fn default() -> Level {
        Self::Qm
    }
}
/// Current competition status for the playoffs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "won")]
    Won,
    #[serde(rename = "eliminated")]
    Eliminated,
    #[serde(rename = "playing")]
    Playing,
}

impl Default for Status {
    fn default() -> Status {
        Self::Won
    }
}

