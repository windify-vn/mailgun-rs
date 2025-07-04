use crate::framework::response::ApiResult;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SendEmailResponse {
    pub id: String,
    pub message: String,
}

impl ApiResult for SendEmailResponse {}

#[derive(Debug, Deserialize, Clone)]
pub struct GetStoredEmailResponse {
    #[serde(rename = "Content-Transfer-Encoding")]
    pub content_transfer_encoding: String,
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    #[serde(rename = "From")]
    pub form: String,
    #[serde(rename = "Message-Id")]
    pub message_id: String,
    #[serde(rename = "Mime-Version")]
    pub mime_version: String,
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "X-Mailgun-Tag")]
    pub mailgun_tags: String,
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "recipients")]
    pub recipients: String,
    #[serde(rename = "body-html")]
    pub body_html: String,
    #[serde(rename = "body-plain")]
    pub body_plain: String,
    #[serde(rename = "stripped-html")]
    pub stripped_html: String,
    #[serde(rename = "stripped-text")]
    pub stripped_text: String,
    #[serde(rename = "stripped-signature")]
    pub stripped_signature: String,
    #[serde(rename = "message-headers")]
    pub message_headers: Vec<Vec<String>>,
    #[serde(rename = "X-Mailgun-Template-Name")]
    pub mailgun_template_name: String,
    #[serde(rename = "X-Mailgun-Template-Variables")]
    pub mailgun_template_variables: String,
}

impl ApiResult for GetStoredEmailResponse {}
