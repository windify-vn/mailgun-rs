use crate::endpoints::messages::messages::{Attachment, AttachmentType, EmailAddress, Template};
use crate::framework::endpoint::MultipartPart;
use std::fs;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Debug, PartialEq, Eq, Clone)]
pub struct SendEmailRequest {
    #[builder(setter(into))]
    pub(crate) domain: String,

    #[builder(setter(into))]
    pub(crate) from: Vec<EmailAddress>,
    #[builder(setter(into))]
    pub(crate) to: Vec<EmailAddress>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) subject: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) cc: Option<Vec<EmailAddress>>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) bcc: Option<Vec<EmailAddress>>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) text: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) html: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) amp_html: Option<String>,

    #[builder(default, setter(into))]
    pub(crate) attachments: Vec<Attachment>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) template: Option<Template>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) tags: Option<Vec<String>>,

    #[builder(default, setter(strip_option))]
    pub(crate) dkim: Option<bool>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) secondary_dkim: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) secondary_dkim_public: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) delivery_time: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) delivery_time_optimize_period: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) timezone_localize: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) test_mode: Option<bool>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) tracking: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) tracking_clicks: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) tracking_opens: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) require_tls: Option<bool>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) skip_verification: Option<bool>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) sending_ip: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) sending_ip_pool: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) tracking_pixel_location_top: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) custom_headers: Option<Vec<(String, String)>>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) custom_variables: Option<Vec<(String, serde_json::Value)>>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) recipient_variables: Option<serde_json::Value>,
}

impl crate::framework::endpoint::MultipartBody for SendEmailRequest {
    fn parts(&self) -> Vec<(String, MultipartPart)> {
        let mut parts: Vec<(String, MultipartPart)> = vec![];

        parts.push((
            "from".into(),
            MultipartPart::Text(EmailAddress::payload_string(&self.from)),
        ));
        parts.push((
            "to".into(),
            MultipartPart::Text(EmailAddress::payload_string(&self.to)),
        ));

        if let Some(value) = &self.subject {
            parts.push(("subject".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.cc {
            parts.push((
                "cc".into(),
                MultipartPart::Text(EmailAddress::payload_string(value)),
            ));
        }

        if let Some(value) = &self.bcc {
            parts.push((
                "bcc".into(),
                MultipartPart::Text(EmailAddress::payload_string(value)),
            ));
        }

        if let Some(value) = &self.text {
            parts.push(("text".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.html {
            parts.push(("html".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.amp_html {
            parts.push(("amp-html".into(), MultipartPart::Text(value.into())));
        }

        for attachment in &self.attachments {
            if let Ok(data) = fs::read(&attachment.path) {
                parts.push((
                    attachment.attachment_type.to_string(),
                    MultipartPart::Bytes(data),
                ))
            }
        }

        if let Some(template) = &self.template {
            parts.push((
                "template".into(),
                MultipartPart::Text(template.name.as_str().into()),
            ));

            if let Some(value) = &template.version {
                parts.push(("t:version".into(), MultipartPart::Text(value.into())));
            }

            if let Some(text) = &template.text {
                parts.push(("t:text".into(), MultipartPart::Text(text.into())));
            }

            if let Some(value) = &template.variables {
                parts.push((
                    "h:X-Mailgun-Variables".into(),
                    MultipartPart::Text(serde_json::to_string(value).unwrap_or_default()),
                ));
            }
        }

        if let Some(value) = &self.tags {
            parts.push(("o:tag".into(), MultipartPart::Text(value.join(","))));
        }

        if let Some(value) = &self.dkim {
            let value = if *value { "yes" } else { "no" };

            parts.push(("dkim".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.secondary_dkim {
            parts.push(("o:secondary-dkim".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.secondary_dkim_public {
            parts.push((
                "o:secondary-dkim-public".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.delivery_time {
            parts.push(("o:deliverytime".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.delivery_time_optimize_period {
            parts.push((
                "o:deliverytime-optimize-period".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.timezone_localize {
            parts.push((
                "o:time-zone-localize".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.test_mode {
            let value = if *value { "yes" } else { "no" };

            parts.push(("o:testmode".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.tracking {
            parts.push(("o:tracking".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.tracking_clicks {
            parts.push((
                "o:tracking-clicks".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.tracking_opens {
            parts.push(("o:tracking-opens".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.require_tls {
            let value = if *value { "yes" } else { "no" };

            parts.push(("o:require-tls".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.skip_verification {
            let value = if *value { "yes" } else { "no" };

            parts.push((
                "o:skip-verification".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.sending_ip {
            parts.push(("o:sending-ip".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.sending_ip_pool {
            parts.push((
                "o:sending-ip-pool".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.tracking_pixel_location_top {
            parts.push((
                "o:tracking-pixel-location-top".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.custom_headers {
            for (key, value) in value {
                parts.push((format!("h:X-{key}"), MultipartPart::Text(value.into())));
            }
        }

        if let Some(value) = &self.custom_variables {
            for (key, value) in value {
                parts.push((
                    format!("v:{key}"),
                    MultipartPart::Text(serde_json::to_string(value).unwrap_or_default()),
                ));
            }
        }

        if let Some(value) = &self.recipient_variables {
            parts.push((
                "recipient-variables".into(),
                MultipartPart::Text(serde_json::to_string(value).unwrap_or_default()),
            ));
        }

        parts
    }
}

#[derive(TypedBuilder, Default, Debug, PartialEq, Eq, Clone)]
pub struct GetStoredEmailRequest {
    #[builder(setter(into))]
    pub(crate) key: String,
    #[builder(setter(into))]
    pub(crate) domain: String,
}

#[derive(TypedBuilder, Default, Debug, PartialEq, Eq, Clone)]
pub struct GetSendingQueueInfoRequest {
    #[builder(setter(into))]
    pub(crate) domain: String,
}
