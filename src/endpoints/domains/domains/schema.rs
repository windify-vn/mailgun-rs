use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum DomainState {
    Active,
    Unverified,
    Disabled,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DomainDetail {
    pub created_at: String,
    pub id: String,
    pub is_disabled: bool,
    pub name: String,
    pub require_tls: bool,
    pub skip_verification: bool,
    pub smtp_login: String,
    pub spam_action: String,
    pub state: DomainState,
    #[serde(rename = "type")]
    pub domain_type: String,
    pub use_automatic_sender_security: bool,
    pub web_prefix: String,
    pub web_scheme: String,
    pub wildcard: bool,
    pub encrypt_incoming_message: bool,
    pub smtp_password: Option<String>,
    pub subaccount_id: Option<String>,
    pub tracking_host: Option<String>,
    #[serde(rename = "disabled")]
    pub disabled_detail: Option<DomainDisabledDetail>,
    pub message_ttl: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DomainDisabledDetail {
    pub code: String,
    pub note: String,
    pub permanently: bool,
    pub reason: String,
    pub util: Option<String>,
}
