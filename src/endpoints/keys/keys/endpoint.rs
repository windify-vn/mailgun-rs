use crate::endpoints::keys::keys::{GetKeysListRequest, GetKeysListResponse};
use crate::framework::endpoint::EndpointSpec;
use http::Method;

impl EndpointSpec for GetKeysListRequest {
    type ResponseType = GetKeysListResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "v1/keys".into()
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serde_urlencoded::to_string(self).ok()
    }
}
