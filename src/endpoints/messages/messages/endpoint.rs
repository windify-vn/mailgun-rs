use crate::endpoints::GenericResponse;
use crate::endpoints::messages::messages::{
    ClearMessageQueueRequest, GetSendingQueueInfoRequest, GetSendingQueueInfoResponse,
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

impl EndpointSpec for GetSendingQueueInfoRequest {
    type ResponseType = GetSendingQueueInfoResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("v3/domains/{}/sending_queues", self.domain)
    }
}

impl EndpointSpec for ClearMessageQueueRequest {
    type ResponseType = GenericResponse;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("v3/domains/{}/envelopes", self.domain)
    }
}
