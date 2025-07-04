use crate::endpoints::domains::domains::{
    CreateDomainRequest, CreateDomainResponse, GetDomainListRequest, GetDomainListResponse,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;
use url::UrlQuery;

impl EndpointSpec for GetDomainListRequest {
    type ResponseType = GetDomainListResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "v4/domains".into()
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serde_urlencoded::to_string(self).ok()
    }
}

impl EndpointSpec for CreateDomainRequest {
    type ResponseType = CreateDomainResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "v4/domains".into()
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::MultiPart(self))
    }
}
