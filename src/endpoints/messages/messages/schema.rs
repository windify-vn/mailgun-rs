use std::fmt;
use std::fmt::Formatter;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug, PartialEq, Eq, Clone)]
pub struct EmailAddress {
    name: Option<String>,
    address: String,
}

impl EmailAddress {
    pub fn address(address: &str) -> Self {
        EmailAddress {
            name: None,
            address: address.to_string(),
        }
    }

    pub fn name_address(name: &str, address: &str) -> Self {
        EmailAddress {
            name: Some(name.to_string()),
            address: address.to_string(),
        }
    }

    pub(crate) fn payload_string(address: &[EmailAddress]) -> String {
        address
            .iter()
            .map(EmailAddress::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} <{}>", name, self.address),
            None => write!(f, "{}", self.address),
        }
    }
}

impl From<&str> for EmailAddress {
    fn from(address: &str) -> Self {
        EmailAddress::address(address)
    }
}

impl From<(&str, &str)> for EmailAddress {
    fn from((name, address): (&str, &str)) -> Self {
        EmailAddress::name_address(name, address)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttachmentType {
    Attachment,
    Inline,
}

impl fmt::Display for AttachmentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let text = match self {
            AttachmentType::Attachment => "attachment",
            AttachmentType::Inline => "inline",
        };

        write!(f, "{text}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub struct Attachment {
    #[builder(setter(into))]
    pub path: String,
    #[builder(default = AttachmentType::Attachment)]
    pub attachment_type: AttachmentType,
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub struct Template {
    #[builder(setter(into))]
    pub(crate) name: String,
    #[builder(default, setter(strip_option, into))]
    pub(crate) version: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) text: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub(crate) variables: Option<serde_json::Value>,
}
