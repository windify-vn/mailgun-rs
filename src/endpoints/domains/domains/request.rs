use crate::endpoints::domains::domains::DomainState;
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
