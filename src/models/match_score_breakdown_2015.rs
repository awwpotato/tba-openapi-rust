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

/// MatchScoreBreakdown2015 : See the 2015 FMS API documentation for a description of each value
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchScoreBreakdown2015 {
    #[serde(rename = "blue")]
    pub blue: Box<models::MatchScoreBreakdown2015Alliance>,
    #[serde(rename = "red")]
    pub red: Box<models::MatchScoreBreakdown2015Alliance>,
    #[serde(rename = "coopertition")]
    pub coopertition: Coopertition,
    #[serde(rename = "coopertition_points")]
    pub coopertition_points: i32,
}

impl MatchScoreBreakdown2015 {
    /// See the 2015 FMS API documentation for a description of each value
    pub fn new(blue: models::MatchScoreBreakdown2015Alliance, red: models::MatchScoreBreakdown2015Alliance, coopertition: Coopertition, coopertition_points: i32) -> MatchScoreBreakdown2015 {
        MatchScoreBreakdown2015 {
            blue: Box::new(blue),
            red: Box::new(red),
            coopertition,
            coopertition_points,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Coopertition {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Stack")]
    Stack,
}

impl Default for Coopertition {
    fn default() -> Coopertition {
        Self::None
    }
}

