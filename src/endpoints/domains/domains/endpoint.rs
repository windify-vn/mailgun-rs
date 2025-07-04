use crate::endpoints::domains::domains::{GetDomainListRequest, GetDomainListResponse};
use crate::framework::endpoint::EndpointSpec;
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
