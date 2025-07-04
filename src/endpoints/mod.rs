use crate::framework::response::ApiResult;
use serde::Deserialize;

#[allow(unused_imports)]
pub mod messages;

#[derive(Debug, Deserialize, Clone)]
pub struct GenericResponse {
    pub message: String,
}

impl ApiResult for GenericResponse {}
