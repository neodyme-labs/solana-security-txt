use crate::{SECURITY_TXT_BEGIN, SECURITY_TXT_END};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{self, Display},
    str::from_utf8,
};
use hashbrown::HashMap;
use thiserror::Error;
use twoway::find_bytes;

pub enum Contact {
    Email(String),
    Discord(String),
    Telegram(String),
    Twitter(String),
    Link(String),
    Other(String),
}

impl Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Contact::Discord(s) => write!(f, "Discord: {}", s),
            Contact::Email(s) => write!(f, "Email: {}", s),
            Contact::Telegram(s) => write!(f, "Telegram: {}", s),
            Contact::Twitter(s) => write!(f, "Twitter: {}", s),
            Contact::Link(s) => write!(f, "Link: {}", s),
            Contact::Other(s) => write!(f, "Other: {}", s),
        }
    }
}

pub struct SecurityTxt {
    pub name: String,
    pub project_url: String,
    pub contacts: Vec<Contact>,
    pub policy: String,
    pub preferred_languages: Vec<String>,
    pub source_code: Option<String>,
    pub source_release: Option<String>,
    pub source_revision: Option<String>,
    pub encryption: Option<String>,
    pub auditors: Vec<String>,
    pub acknowledgements: Option<String>,
    pub expiry: Option<String>,
}

impl Display for SecurityTxt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Project URL: {}", self.project_url)?;

        if !self.contacts.is_empty() {
            writeln!(f, "\nContacts:")?;
            for contact in &self.contacts {
                writeln!(f, "  {}", contact)?;
            }
        }

        writeln!(f, "\nPolicy:")?;
        writeln!(f, "{}", self.policy)?;

        if !self.preferred_languages.is_empty() {
            writeln!(f, "\nPreferred Languages:")?;
            for languages in &self.preferred_languages {
                writeln!(f, "  {}", languages)?;
            }
        }

        if let Some(source_code) = &self.source_code {
            writeln!(f, "Source code: {}", source_code)?;
        }

        if let Some(source_release) = &self.source_release {
            writeln!(f, "Source release: {}", source_release)?;
        }

        if let Some(source_revision) = &self.source_revision {
            writeln!(f, "Source revision: {}", source_revision)?;
        }

        if let Some(encryption) = &self.encryption {
            writeln!(f, "\nEncryption:")?;
            writeln!(f, "{}", encryption)?;
        }

        if !self.auditors.is_empty() {
            writeln!(f, "\nAuditors:")?;
            for auditor in &self.auditors {
                writeln!(f, "  {}", auditor)?;
            }
        }

        if let Some(acknowledegments) = &self.acknowledgements {
            writeln!(f, "\nAcknowledgements:")?;
            writeln!(f, "{}", acknowledegments)?;
        }

        if let Some(expiry) = &self.expiry {
            writeln!(f, "Expires at: {}", expiry)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Contact {
    type Error = SecurityTxtError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (typ, value) = s
            .split_once(":")
            .ok_or_else(|| SecurityTxtError::InvalidContact(s.to_string()))?;
        let (contact_type, contact_info) = (typ.trim(), value.trim());
        match contact_type.to_ascii_lowercase().as_str() {
            "email" => Ok(Contact::Email(contact_info.to_string())),
            "discord" => Ok(Contact::Discord(contact_info.to_string())),
            "telegram" => Ok(Contact::Telegram(contact_info.to_string())),
            "twitter" => Ok(Contact::Twitter(contact_info.to_string())),
            "link" => Ok(Contact::Link(contact_info.to_string())),
            "other" => Ok(Contact::Other(contact_info.to_string())),
            _ => Err(SecurityTxtError::InvalidContact(s.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum SecurityTxtError {
    #[error("security.txt doesn't start with the right string")]
    InvalidSecurityTxtBegin,
    #[error("Couldn't find end string")]
    EndNotFound,
    #[error("Couldn't find start string")]
    StartNotFound,
    #[error("Invalid field: `{0:?}`")]
    InvalidField(Vec<u8>),
    #[error("Unknown field: `{0}`")]
    UnknownField(String),
    #[error("Invalid value `{0:?}` for field `{1}`")]
    InvalidValue(Vec<u8>, String),
    #[error("Invalid contact `{0}`")]
    InvalidContact(String),
    #[error("Missing field: `{0}`")]
    MissingField(String),
    #[error("Duplicate field: `{0}`")]
    DuplicateField(String),
    #[error("Uneven amount of parts")]
    Uneven,
}
/// Parses a security.txt. Might not consume all of `data`.
pub fn parse(mut data: &[u8]) -> Result<SecurityTxt, SecurityTxtError> {
    if !data.starts_with(SECURITY_TXT_BEGIN.as_bytes()) {
        return Err(SecurityTxtError::InvalidSecurityTxtBegin);
    }

    let end = match find_bytes(data, SECURITY_TXT_END.as_bytes()) {
        Some(i) => i,
        None => return Err(SecurityTxtError::EndNotFound),
    };

    data = &data[SECURITY_TXT_BEGIN.len()..end];

    let mut attributes = HashMap::<String, String>::default();
    let mut field: Option<String> = None;
    for part in data.split(|&b| b == 0) {
        if let Some(ref f) = field {
            let value = from_utf8(part)
                .map_err(|_| SecurityTxtError::InvalidValue(part.to_vec(), f.clone()))?;
            attributes.insert(f.clone(), value.to_string());
            field = None;
        } else {
            field = Some({
                let field = from_utf8(part)
                    .map_err(|_| SecurityTxtError::InvalidField(part.to_vec()))?
                    .to_string();
                if attributes.contains_key(&field) {
                    return Err(SecurityTxtError::DuplicateField(field));
                }
                field
            });
        }
    }

    let name = attributes
        .remove("name")
        .ok_or_else(|| SecurityTxtError::MissingField("name".to_string()))?;
    let project_url = attributes
        .remove("project_url")
        .ok_or_else(|| SecurityTxtError::MissingField("project_url".to_string()))?;
    let source_code = attributes.remove("source_code");
    let source_release = attributes.remove("source_release");
    let source_revision = attributes.remove("source_revision");
    let expiry = attributes.remove("expiry");
    let preferred_languages = attributes
        .remove("preferred_languages")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    let contacts: Result<Vec<_>, SecurityTxtError> = attributes
        .remove("contacts")
        .ok_or_else(|| SecurityTxtError::MissingField("contacts".to_string()))?
        .split(",")
        .map(|s| Contact::try_from(s.trim()))
        .collect();
    let contacts = contacts?;
    let auditors: Vec<_> = attributes
        .remove("auditors")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    let encryption = attributes.remove("encryption");
    let acknowledgements = attributes.remove("acknowledgements");
    let policy = attributes
        .remove("policy")
        .ok_or_else(|| SecurityTxtError::MissingField("policy".to_string()))?;

    if !attributes.is_empty() {
        return Err(SecurityTxtError::UnknownField(
            attributes.keys().next().unwrap().clone(),
        ));
    }

    Ok(SecurityTxt {
        name,
        project_url,
        source_code,
        source_release,
        source_revision,
        expiry,
        preferred_languages,
        contacts,
        auditors,
        encryption,
        acknowledgements,
        policy,
    })
}

/// Finds and parses the security.txt in the haystack
pub fn find_and_parse(data: &[u8]) -> Result<SecurityTxt, SecurityTxtError> {
    let start = match find_bytes(data, SECURITY_TXT_BEGIN.as_bytes()) {
        Some(i) => i,
        None => return Err(SecurityTxtError::StartNotFound),
    };
    parse(&data[start..])
}
