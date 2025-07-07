use crate::framework::MailgunRegion;
use crate::framework::auth::Credentials;
use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::{ApiErrors, ApiFailure, ApiResponse};
use std::borrow::Cow;
use std::net::SocketAddr;
use crate::framework::client::async_api::Client;

pub struct HttpApiClient {
    region: MailgunRegion,
    credentials: Credentials,
    http_client: reqwest::blocking::Client,
}

impl HttpApiClient {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        region: MailgunRegion,
    ) -> Result<HttpApiClient, crate::framework::Error> {
        let mut builder = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers);

        if let Some(address) = config.resolve_ip {
            let url = url::Url::from(&region);
            builder = builder.resolve(
                url.host_str().expect("Region url should have a hostname"),
                SocketAddr::new(address, 443),
            );
        }
        let http_client = builder.build()?;

        Ok(HttpApiClient {
            region,
            credentials,
            http_client,
        })
    }

    pub fn new_with_client(
        client: reqwest::blocking::Client,
        credentials: Credentials,
        region: MailgunRegion,
    ) -> HttpApiClient {
        HttpApiClient {
            region,
            credentials,
            http_client: client,
        }
    }

    pub fn request<Endpoint>(&self, endpoint: &Endpoint) -> ApiResponse<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(endpoint.method(), endpoint.url(&self.region));

        if let Some(body) = endpoint.body() {
            match body {
                RequestBody::Json(json) => {
                    request = request.body(json);
                }
                RequestBody::Raw(bytes) => {
                    request = request.body(bytes);
                }
                RequestBody::MultiPart(multipart) => {
                    let mut form = reqwest::blocking::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form
                                    .part(name, reqwest::blocking::multipart::Part::bytes(bytes));
                            }
                        }
                    }
                    request = request.multipart(form);
                }
            }
            // Reqwest::RequestBuilder::multipart sets the content type for us.
            match endpoint.content_type() {
                None | Some(Cow::Borrowed("multipart/form-data")) => {}
                Some(content_type) => {
                    request = request.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());
                }
            }
        }

        let response = request
            .basic_auth("api", Some(&self.credentials.key))
            .send()?;

        map_api_response_json::<Endpoint>(response)
    }
}

fn map_api_response_json<Endpoint>(
    resp: reqwest::blocking::Response,
) -> Result<Endpoint::ResponseType, ApiFailure>
where
    Endpoint: EndpointSpec,
{
    let status = resp.status();
    if status.is_success() {
        let parsed: Result<Endpoint::ResponseType, reqwest::Error> = resp.json();
        match parsed {
            Ok(success) => Ok(success),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}
