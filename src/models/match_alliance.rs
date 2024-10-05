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
pub struct MatchAlliance {
    /// Score for this alliance. Will be null or -1 for an unplayed match.
    #[serde(rename = "score")]
    pub score: i32,
    #[serde(rename = "team_keys")]
    pub team_keys: Vec<String>,
    /// TBA team keys (eg `frc254`) of any teams playing as a surrogate.
    #[serde(rename = "surrogate_team_keys")]
    pub surrogate_team_keys: Vec<String>,
    /// TBA team keys (eg `frc254`) of any disqualified teams.
    #[serde(rename = "dq_team_keys")]
    pub dq_team_keys: Vec<String>,
}

impl MatchAlliance {
    pub fn new(score: i32, team_keys: Vec<String>, surrogate_team_keys: Vec<String>, dq_team_keys: Vec<String>) -> MatchAlliance {
        MatchAlliance {
            score,
            team_keys,
            surrogate_team_keys,
            dq_team_keys,
        }
    }
}

