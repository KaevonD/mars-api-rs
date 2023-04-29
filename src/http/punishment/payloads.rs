use serde::{Serialize, Deserialize};

use crate::database::models::{punishment::{PunishmentReason, PunishmentAction}, player::SimplePlayer};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PunishmentIssueRequest {
    pub reason: PunishmentReason,
    pub offence: u32,
    pub action: PunishmentAction,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub punisher: Option<SimplePlayer>,
    pub target_name: String,
    pub target_ips: Vec<String>,
    pub silent: bool
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PunishmentRevertRequest {
    pub reason: String,
    pub reverter: SimplePlayer
}
