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

/// DistrictRanking : Rank of a team in a district.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistrictRanking {
    /// TBA team key for the team.
    #[serde(rename = "team_key")]
    pub team_key: String,
    /// Numerical rank of the team, 1 being top rank.
    #[serde(rename = "rank")]
    pub rank: i32,
    /// Any points added to a team as a result of the rookie bonus.
    #[serde(rename = "rookie_bonus", skip_serializing_if = "Option::is_none")]
    pub rookie_bonus: Option<i32>,
    /// Total district points for the team.
    #[serde(rename = "point_total")]
    pub point_total: i32,
    /// List of events that contributed to the point total for the team.
    #[serde(rename = "event_points", skip_serializing_if = "Option::is_none")]
    pub event_points: Option<Vec<models::DistrictRankingEventPointsInner>>,
}

impl DistrictRanking {
    /// Rank of a team in a district.
    pub fn new(team_key: String, rank: i32, point_total: i32) -> DistrictRanking {
        DistrictRanking {
            team_key,
            rank,
            rookie_bonus: None,
            point_total,
            event_points: None,
        }
    }
}

