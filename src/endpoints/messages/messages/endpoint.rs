use crate::endpoints::messages::messages::{SendMessageRequest, SendMessageResponse};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for SendMessageRequest {
    type ResponseType = SendMessageResponse;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("v3/{}/messages", self.domain)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::MultiPart(self))
    }
}
