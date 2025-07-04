use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum KeyKind {
    Domain,
    User,
    Web,
    Public,
}

impl fmt::Display for KeyKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            KeyKind::Domain => "domain",
            KeyKind::User => "user",
            KeyKind::Web => "web",
            KeyKind::Public => "public",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum KeyRole {
    #[serde(rename = "")]
    Nil,
    Admin,
    #[default]
    Basic,
    Sending,
    Support,
    Developer,
}

impl fmt::Display for KeyRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            KeyRole::Nil => "",
            KeyRole::Admin => "admin",
            KeyRole::Basic => "basic",
            KeyRole::Sending => "sending",
            KeyRole::Support => "support",
            KeyRole::Developer => "developer",
        };
        write!(f, "{s}")
    }
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
