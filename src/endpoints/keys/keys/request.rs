use crate::endpoints::domains::domains::DomainState;
use crate::endpoints::keys::keys::KeyKind;
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
