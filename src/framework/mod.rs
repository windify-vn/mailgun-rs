/*!
This module controls how requests are sent to Cloudflare's API, and how responses are parsed from it.
 */
pub mod auth;
pub mod client;
pub mod endpoint;
pub mod response;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error via the `reqwest` crate
    #[error("Reqwest returned an error when connecting to the Cloudflare API: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug)]
pub enum MailgunRegion {
    US,
    EU,
    Custom(String),
}

impl From<&MailgunRegion> for url::Url {
    fn from(region: &MailgunRegion) -> Self {
        match region {
            MailgunRegion::US => url::Url::parse("https://api.mailgun.net").unwrap(),
            MailgunRegion::EU => url::Url::parse("https://api.eu.mailgun.net").unwrap(),
            MailgunRegion::Custom(url) => url::Url::parse(url.as_str()).unwrap(),
        }
    }
}
