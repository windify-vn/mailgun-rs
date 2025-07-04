use crate::endpoints::keys::keys::{KeyKind, KeyRole};
use crate::framework::endpoint::MultipartPart;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct GetKeysListRequest {
    #[builder(default, setter(strip_option, into))]
    pub(crate) domain_name: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) kind: Option<KeyKind>,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct CreateKeyRequest {
    #[builder(setter(into))]
    pub(crate) role: KeyRole,
    #[builder(default, setter(strip_option, into))]
    pub(crate) email: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) domain_name: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) kind: Option<KeyKind>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) expiration: Option<u64>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) user_id: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) user_name: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub(crate) description: Option<String>,
}

impl crate::framework::endpoint::MultipartBody for CreateKeyRequest {
    fn parts(&self) -> Vec<(String, MultipartPart)> {
        let mut parts: Vec<(String, MultipartPart)> = vec![];

        parts.push(("role".into(), MultipartPart::Text(self.role.to_string())));

        if let Some(value) = &self.email {
            parts.push(("email".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.domain_name {
            parts.push(("domain_name".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.kind {
            parts.push(("kind".into(), MultipartPart::Text(value.to_string())));
        }

        if let Some(value) = &self.expiration {
            parts.push(("expiration".into(), MultipartPart::Text(value.to_string())));
        }

        if let Some(value) = &self.user_id {
            parts.push(("user_id".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.user_name {
            parts.push(("user_name".into(), MultipartPart::Text(value.into())));
        }

        if let Some(value) = &self.description {
            parts.push(("description".into(), MultipartPart::Text(value.into())));
        }

        parts
    }
}
