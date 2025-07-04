use crate::endpoints::GenericResponse;
use crate::endpoints::domains::domains::{
    CreateDomainRequest, CreateDomainResponse, DeleteDomainRequest, GetDomainDmarcRequest,
    GetDomainDmarcResponse, GetDomainListRequest, GetDomainListResponse, UpdateDomainRequest,
    UpdateDomainResponse, VerifyDomainRequest, VerifyDomainResponse,
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

impl EndpointSpec for UpdateDomainRequest {
    type ResponseType = UpdateDomainResponse;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!("v4/domains/{}", self.domain)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::MultiPart(self))
    }
}

impl EndpointSpec for VerifyDomainRequest {
    type ResponseType = VerifyDomainResponse;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!("v4/domains/{}/verify", self.domain)
    }
}

impl EndpointSpec for DeleteDomainRequest {
    type ResponseType = GenericResponse;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("v3/domains/{}", self.domain)
    }
}

impl EndpointSpec for GetDomainDmarcRequest {
    type ResponseType = GetDomainDmarcResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("v1/dmarc/records/{}", self.domain)
    }
}
