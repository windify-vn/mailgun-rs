use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, strum_macros::AsRefStr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum KeyKind {
    Domain,
    User,
    Web,
    Public,
}

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum_macros::AsRefStr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum KeyRole {
    #[serde(rename = "")]
    #[strum(serialize = "")]
    Nil,
    Admin,
    #[default]
    Basic,
    Sending,
    Support,
    Developer,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyDetail {
    pub id: String,
    pub description: String,
    pub kind: KeyKind,
    pub role: KeyRole,
    pub created_at: String,
    pub updated_at: String,
    pub domain_name: Option<String>,
    pub requestor: Option<String>,
    pub user_name: Option<String>,
    pub is_disabled: bool,
    pub expires_at: Option<String>,
    pub secret: Option<String>,
    pub disabled_reason: Option<String>,
}
