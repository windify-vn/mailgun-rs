use crate::endpoints::domains::domains::DomainState;
use crate::endpoints::messages::messages::EmailAddress;
use crate::framework::endpoint::MultipartPart;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct GetDomainListRequest {
    #[builder(default, setter(strip_option, into))]
    pub(crate) limit: Option<u16>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) skip: Option<u32>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) state: Option<DomainState>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) sort: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) authority: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) search: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) include_subaccounts: Option<bool>,
}

#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct CreateDomainRequest {
    #[builder(setter(into))]
    pub(crate) domain: String,
    #[builder(default, setter(strip_option, into))]
    pub(crate) dkim_host_name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub(crate) dkim_key_size: Option<u32>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) dkim_selector: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) encrypt_incoming_message: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) force_dkim_authority: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) force_root_dkim_host: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) wildcard: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) pool_id: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) ips: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) require_tls: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) skip_verification: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) spam_action: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) smtp_password: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) use_automatic_sender_security: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) web_prefix: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) web_scheme: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) message_ttl: Option<i32>,
}

impl crate::framework::endpoint::MultipartBody for CreateDomainRequest {
    fn parts(&self) -> Vec<(String, MultipartPart)> {
        let mut parts: Vec<(String, MultipartPart)> = vec![];

        parts.push(("name".into(), MultipartPart::Text(self.domain.clone())));

        if let Some(value) = &self.dkim_host_name {
            parts.push(("dkim_host_name".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.dkim_key_size {
            parts.push((
                "dkim_key_size".into(),
                MultipartPart::Text(value.to_string()),
            ));
        }

        if let Some(value) = &self.dkim_selector {
            parts.push(("dkim_selector".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.encrypt_incoming_message {
            let value = if *value { "true" } else { "false" };
            parts.push((
                "encrypt_incoming_message".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.force_dkim_authority {
            let value = if *value { "true" } else { "false" };
            parts.push((
                "force_dkim_authority".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.force_root_dkim_host {
            let value = if *value { "true" } else { "false" };
            parts.push((
                "force_root_dkim_host".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.wildcard {
            let value = if *value { "true" } else { "false" };
            parts.push(("wildcard".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.pool_id {
            parts.push(("pool_id".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.ips {
            parts.push(("ips".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.require_tls {
            let value = if *value { "true" } else { "false" };
            parts.push(("require_tls".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.skip_verification {
            let value = if *value { "true" } else { "false" };
            parts.push((
                "skip_verification".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.spam_action {
            parts.push(("spam_action".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.smtp_password {
            parts.push(("smtp_password".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.use_automatic_sender_security {
            let value = if *value { "true" } else { "false" };
            parts.push((
                "use_automatic_sender_security".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.web_prefix {
            parts.push(("web_prefix".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.web_scheme {
            parts.push(("web_scheme".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.message_ttl {
            parts.push(("message_ttl".into(), MultipartPart::Text(value.to_string())));
        }

        parts
    }
}

#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct UpdateDomainRequest {
    #[builder(setter(into))]
    pub(crate) domain: String,
    #[builder(default, setter(strip_option, into))]
    pub(crate) mailfrom_host: Option<String>,
    #[builder(default, setter(strip_option))]
    pub(crate) message_ttl: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub(crate) require_tls: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) skip_verification: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) smtp_password: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) spam_action: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) use_automatic_sender_security: Option<bool>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) web_scheme: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) web_prefix: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) wildcard: Option<bool>,
}

impl crate::framework::endpoint::MultipartBody for UpdateDomainRequest {
    fn parts(&self) -> Vec<(String, MultipartPart)> {
        let mut parts: Vec<(String, MultipartPart)> = vec![];

        if let Some(value) = &self.mailfrom_host {
            parts.push(("mailfrom_host".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.message_ttl {
            parts.push(("message_ttl".into(), MultipartPart::Text(value.to_string())));
        }

        if let Some(value) = &self.require_tls {
            let value = if *value { "true" } else { "false" };
            parts.push(("require_tls".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.skip_verification {
            let value = if *value { "true" } else { "false" };
            parts.push((
                "skip_verification".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.spam_action {
            parts.push(("spam_action".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.smtp_password {
            parts.push(("smtp_password".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.use_automatic_sender_security {
            let value = if *value { "true" } else { "false" };
            parts.push((
                "use_automatic_sender_security".into(),
                MultipartPart::Text(value.into()),
            ));
        }

        if let Some(value) = &self.web_scheme {
            parts.push(("web_scheme".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.web_prefix {
            parts.push(("web_prefix".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.wildcard {
            let value = if *value { "true" } else { "false" };
            parts.push(("wildcard".into(), MultipartPart::Text(value.into())));
        }

        parts
    }
}

#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct VerifyDomainRequest {
    #[builder(setter(into))]
    pub(crate) domain: String,
}
