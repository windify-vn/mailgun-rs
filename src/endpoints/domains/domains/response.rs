use crate::endpoints::domains::domains::{DomainDetail, DomainDnsRecord};
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetDomainListResponse {
    pub total_count: i32,
    pub items: Vec<DomainDetail>,
}

impl ApiResult for GetDomainListResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDomainResponse {
    pub message: String,
    pub domain: DomainDetail,
    pub receiving_dns_records: Vec<DomainDnsRecord>,
    pub sending_dns_records: Vec<DomainDnsRecord>,
}

impl ApiResult for CreateDomainResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateDomainResponse {
    pub message: String,
    pub domain: DomainDetail,
    pub receiving_dns_records: Option<Vec<DomainDnsRecord>>,
    pub sending_dns_records: Option<Vec<DomainDnsRecord>>,
}

impl ApiResult for UpdateDomainResponse {}
