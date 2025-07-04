use crate::endpoints::GenericResponse;
use crate::endpoints::keys::keys::{
    CreateKeyRequest, CreateKeyResponse, DeleteKeyRequest, GetKeysListRequest, GetKeysListResponse,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
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

impl EndpointSpec for CreateKeyRequest {
    type ResponseType = CreateKeyResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "v1/keys".into()
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::MultiPart(self))
    }
}

impl EndpointSpec for DeleteKeyRequest {
    type ResponseType = GenericResponse;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("v1/keys/{}", self.key_id)
    }
}
