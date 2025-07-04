use crate::endpoints::keys::keys::KeyDetail;
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetKeysListResponse {
    pub total_count: i32,
    pub items: Vec<KeyDetail>,
}

impl ApiResult for GetKeysListResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateKeyResponse {
    pub message: String,
    pub key: KeyDetail,
}

impl ApiResult for CreateKeyResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegeneratePublicKeyResponse {
    pub key: String,
    pub message: String,
}

impl ApiResult for RegeneratePublicKeyResponse {}
