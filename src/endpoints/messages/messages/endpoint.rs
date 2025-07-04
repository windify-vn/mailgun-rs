use crate::endpoints::messages::messages::{
    GetStoredEmailRequest, GetStoredEmailResponse, SendEmailRequest, SendEmailResponse,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for SendEmailRequest {
    type ResponseType = SendEmailResponse;

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

impl EndpointSpec for GetStoredEmailRequest {
    type ResponseType = GetStoredEmailResponse;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("v3/domains/{}/messages/{}", self.domain, self.key)
    }
}
