use crate::endpoints::domains::domains::DomainDetail;
use crate::framework::response::ApiResult;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GetDomainListResponse {
    pub total_count: i32,
    pub items: Vec<DomainDetail>,
}

impl ApiResult for GetDomainListResponse {}
