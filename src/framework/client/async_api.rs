use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::{
    MailgunRegion,
    auth::Credentials,
    response::ApiResponse,
    response::{ApiErrors, ApiFailure},
};
use std::borrow::Cow;
use std::net::SocketAddr;

pub struct Client {
    region: MailgunRegion,
    credentials: Credentials,
    http_client: reqwest::Client,
}
impl Client {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        region: MailgunRegion,
    ) -> Result<Client, crate::framework::Error> {
        let mut builder = reqwest::Client::builder().default_headers(config.default_headers);

        #[cfg(not(target_arch = "wasm32"))]
        {
            // There is no resolve method in wasm.
            if let Some(address) = config.resolve_ip {
                let url = url::Url::from(&region);
                builder = builder.resolve(
                    url.host_str().expect("Region url should have a hostname"),
                    SocketAddr::new(address, 443),
                );
            }

            // There are no timeouts in wasm. The property is documented as no-op in wasm32.
            builder = builder.timeout(config.http_timeout);
        }

        let http_client = builder.build()?;

        Ok(Client {
            region,
            credentials,
            http_client,
        })
    }

    /// Issue an API request of the given type.
    pub async fn request<Endpoint>(
        &self,
        endpoint: &Endpoint,
    ) -> ApiResponse<Endpoint::ResponseType>
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
                    let mut form = reqwest::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form.part(name, reqwest::multipart::Part::bytes(bytes));
                            }
                        }
                    }
                    request = request.multipart(form);
                }
            }
            match endpoint.content_type() {
                None | Some(Cow::Borrowed("multipart/form-data")) => {}
                Some(content_type) => {
                    request = request.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());
                }
            }
        }

        let response = request
            .basic_auth("api", Some(&self.credentials.key))
            .send()
            .await?;

        map_api_response_json::<Endpoint>(response).await
    }
}

async fn map_api_response_json<Endpoint>(
    resp: reqwest::Response,
) -> Result<Endpoint::ResponseType, ApiFailure>
where
    Endpoint: EndpointSpec,
{
    let status = resp.status();
    if status.is_success() {
        let parsed: Result<Endpoint::ResponseType, reqwest::Error> = resp.json().await;
        match parsed {
            Ok(success) => Ok(success),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json().await;
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}
