//! # security.txt
//!
//! This library defines a macro, whose aim it is to provide easy-to-parse information to security researchers that wish to contact the authors of a Solana smart contract.
//! It is inspired by https://securitytxt.org/.
//!
//! ## Example
//! ```rust
//! security_txt! {
//!     name: "Example",
//!     project_url: "http://example.com",
//!     source_code: "https://github.com/example/example",
//!     expiry: "2042-01-01",
//!     preferred_languages: "en,de",
//!     contacts: "email:example@example.com,discord:example#1234",
//!     auditors: "Neodyme",
//!     encryption: "
//! -----BEGIN PGP PUBLIC KEY BLOCK-----
//! Comment: Alice's OpenPGP certificate
//! Comment: https://www.ietf.org/id/draft-bre-openpgp-samples-01.html
//!
//! mDMEXEcE6RYJKwYBBAHaRw8BAQdArjWwk3FAqyiFbFBKT4TzXcVBqPTB3gmzlC/U
//! b7O1u120JkFsaWNlIExvdmVsYWNlIDxhbGljZUBvcGVucGdwLmV4YW1wbGU+iJAE
//! ExYIADgCGwMFCwkIBwIGFQoJCAsCBBYCAwECHgECF4AWIQTrhbtfozp14V6UTmPy
//! MVUMT0fjjgUCXaWfOgAKCRDyMVUMT0fjjukrAPoDnHBSogOmsHOsd9qGsiZpgRnO
//! dypvbm+QtXZqth9rvwD9HcDC0tC+PHAsO7OTh1S1TC9RiJsvawAfCPaQZoed8gK4
//! OARcRwTpEgorBgEEAZdVAQUBAQdAQv8GIa2rSTzgqbXCpDDYMiKRVitCsy203x3s
//! E9+eviIDAQgHiHgEGBYIACAWIQTrhbtfozp14V6UTmPyMVUMT0fjjgUCXEcE6QIb
//! DAAKCRDyMVUMT0fjjlnQAQDFHUs6TIcxrNTtEZFjUFm1M0PJ1Dng/cDW4xN80fsn
//! 0QEA22Kr7VkCjeAEC08VSTeV+QFsmz55/lntWkwYWhmvOgE=
//! =iIGO
//! -----END PGP PUBLIC KEY BLOCK-----
//! ",
//!     acknowledgements: "
//! The following hackers could've stolen all our money but didn't:
//! - Neodyme
//! ",
//!     policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md"
//! }
//! ```
//!
//! ## Format
//! All values need to be string literals that may not contain nullbytes.
//! Naive parsers may fail if the binary contains one of the security.txt delimiters anywhere else
//! (`=======BEGIN SECURITY.TXT V1=======\0` and `=======END SECURITY.TXT V1=======\0`).
//!
//! The following fields are supported, some of which are required for this to be considered a valid security.txt:
//! - `name` (required): The name of the project.
//! -  `project_url` (required): A URL to the project's homepage/dapp.
//! - `source_code` (optional): A URL to the project's source code.
//! - `expiry` (optional): The date the security.txt will expire. The format is YYYY-MM-DD.
//! - `preferred_languages` (required): A comma-separated list of preferred languages.
//! - `contacts` (required): A comma-separated list of contact information in the format `<contact type>:<contact information>`. Possible contact types are `email`, `discord`, `telegram`, `twitter`, `link` and `other`.
//! - `auditors` (optional): A comma-separated list of people or entities that audited this smart contract. Note that this field is self-reported by the author of the program and might not be acurate.
//! - `encryption` (optional): A PGP public key block (or similar) or a link to one
//! - `acknowledgements` (optional): Either a link or a text document containing acknowledgements to security researchers that have found vulnerabilities in the project in the past.
//! - `policy` (required): Either a link or a text document describing the project's security policy. This should describe what kind of bounties your project offers and the terms under which you offer them.
//!
//! ## How it works
//! The macro inserts a `&str` into the `.security.txt` section of the resulting ELF. Because of how Rust strings work, this is a tuple of a pointer to the actual string and the length.
//!
//! The string the macro builds begins with the start marker `=======BEGIN SECURITY.TXT V1=======\0`, and ends with the end marker `=======END SECURITY.TXT V1=======\0`. In between is a list of an even amount of strings, delimited by nullbytes. Every two strings form a key-value-pair.

use core::fmt;
use std::{collections::HashMap, fmt::Display};

use thiserror::Error;
use twoway::find_bytes;

pub const SECURITY_TXT_BEGIN: &str = "=======BEGIN SECURITY.TXT V1=======\0";
pub const SECURITY_TXT_END: &str = "=======END SECURITY.TXT V1=======\0";

#[macro_export]
macro_rules! security_txt {
    ($($name:ident: $value:expr),*) => {
        #[allow(dead_code)]
        #[no_mangle]
        #[link_section = ".security.txt"]
        pub static security_txt: &str = concat! {
            "=======BEGIN SECURITY.TXT V1=======\0",
            $(stringify!($name), "\0", $value, "\0",)*
            "=======END SECURITY.TXT V1=======\0"
        };
    };
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

impl Contact {
    pub fn from_str(s: &str) -> Result<Self, SecurityTxtError> {
        let parts: Vec<_> = s.split(":").collect();
        if parts.len() != 2 {
            return Err(SecurityTxtError::InvalidContact(s.to_string()));
        }
        let (contact_type, contact_info) = (parts[0].trim(), parts[1].trim());
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

pub struct SecurityTxt {
    pub name: String,
    pub project_url: String,
    pub source_code: Option<String>,
    pub expiry: Option<String>,
    pub preferred_languages: Vec<String>,
    pub contacts: Vec<Contact>,
    pub auditors: Vec<String>,
    pub encryption: Option<String>,
    pub acknowledgements: Option<String>,
    pub policy: String,
}

impl Display for SecurityTxt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Project URL: {}", self.project_url)?;

        if let Some(expiry) = &self.expiry {
            writeln!(f, "Expires at: {}", expiry)?;
        }

        if let Some(source_code) = &self.source_code {
            writeln!(f, "Source code: {}", source_code)?;
        }

        if !self.contacts.is_empty() {
            writeln!(f, "\nContacts:")?;
            for contact in &self.contacts {
                writeln!(f, "  {}", contact)?;
            }
        }

        if !self.preferred_languages.is_empty() {
            writeln!(f, "\nPreferred Languages:")?;
            for languages in &self.preferred_languages {
                writeln!(f, "  {}", languages)?;
            }
        }

        if let Some(encryption) = &self.encryption {
            writeln!(f, "\nEncryption:")?;
            writeln!(f, "{}", encryption)?;
        }

        if let Some(acknowledegments) = &self.acknowledgements {
            writeln!(f, "\nAcknowledgements:")?;
            writeln!(f, "{}", acknowledegments)?;
        }

        if !self.auditors.is_empty() {
            writeln!(f, "\nAuditors:")?;
            for auditor in &self.auditors {
                writeln!(f, "  {}", auditor)?;
            }
        }

        writeln!(f, "\nPolicy:")?;
        writeln!(f, "{}", self.policy)?;

        Ok(())
    }
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
            let value = std::str::from_utf8(part)
                .map_err(|_| SecurityTxtError::InvalidValue(part.to_vec(), f.clone()))?;
            attributes.insert(f.clone(), value.to_string());
            field = None;
        } else {
            field = Some({
                let field = std::str::from_utf8(part)
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
    let expiry = attributes.remove("expiry");
    let preferred_languages: Vec<_> = attributes
        .remove("preferred_languages")
        .ok_or_else(|| SecurityTxtError::MissingField("preferred_languages".to_string()))?
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let contacts: Result<Vec<_>, SecurityTxtError> = attributes
        .remove("contacts")
        .ok_or_else(|| SecurityTxtError::MissingField("contacts".to_string()))?
        .split(",")
        .map(|s| Contact::from_str(s.trim()))
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
