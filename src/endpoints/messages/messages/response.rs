use crate::framework::response::ApiResult;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageResponse {
    pub id: String,
    pub message: String,
}

impl ApiResult for SendMessageResponse {}
