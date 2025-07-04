#![allow(unused_imports)]
use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod domains;
pub mod messages;

#[derive(Debug, Deserialize, Clone)]
pub struct GenericResponse {
    pub message: String,
}

impl ApiResult for GenericResponse {}

pub fn create_dmarc_host(domain: &str) -> String {
    format!("_dmarc.{domain}").to_lowercase()
}
