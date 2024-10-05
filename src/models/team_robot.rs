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
pub struct TeamRobot {
    /// Year this robot competed in.
    #[serde(rename = "year")]
    pub year: i32,
    /// Name of the robot as provided by the team.
    #[serde(rename = "robot_name")]
    pub robot_name: String,
    /// Internal TBA identifier for this robot.
    #[serde(rename = "key")]
    pub key: String,
    /// TBA team key for this robot.
    #[serde(rename = "team_key")]
    pub team_key: String,
}

impl TeamRobot {
    pub fn new(year: i32, robot_name: String, key: String, team_key: String) -> TeamRobot {
        TeamRobot {
            year,
            robot_name,
            key,
            team_key,
        }
    }
}

