//! # security.txt
//!
//! This library defines a macro, whose aim it is to provide easy-to-parse information to security researchers that wish to contact the authors of a Solana smart contract.
//! It is inspired by <https://securitytxt.org/>.
//! 
//! For more info, take a look at the projects [README.md](https://github.com/neodyme-labs/solana-security-txt/)
//!
//! ## Example
//! ```rust
//! security_txt! {
//!     // Required fields
//!     name: "Example",
//!     project_url: "http://example.com",
//!     contacts: "email:example@example.com,link:https://example.com/security,discord:example#1234",
//!     policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md",
//! 
//!     // Optional Fields
//!     preferred_languages: "en,de",
//!     source_code: "https://github.com/example/example",
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
//!     auditors: "Neodyme",
//!     acknowledgements: "
//! The following hackers could've stolen all our money but didn't:
//! - Neodyme
//! "
//! }
//! ```
//!
//! ## Format
//! All values need to be string literals that may not contain nullbytes.
//! Naive parsers may fail if the binary contains one of the security.txt delimiters anywhere else
//! (`=======BEGIN SECURITY.TXT V1=======\0` and `=======END SECURITY.TXT V1=======\0`).
//!
//! The following fields are supported, some of which are required for this to be considered a valid security.txt:
//! 
//! | Field                 |         Type         | Description                                                                                                                                                                                                                     |
//! |-----------------------|:--------------------:|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | **`name`**            |   string (required)  | The name of the project. If the project isn't public, you can put `private`.                                                                                                                                                    |
//! | **`project_url`**     | https url (required) | A URL to the project's homepage/dapp. If the project isn't public, you can put `private`.                                                                                                                                       |
//! | **`contacts`**        |    list (required)   | A comma-separated list of contact information in the format `:`. Should roughly be ordered in preference. Possible contact types are `email`, `link`, `discord`, `telegram`, `twitter` and `other`.                             |
//! | **`policy`**          | link/text (required) | Either a link or a text document describing the project's security policy. This should describe what kind of bounties your project offers and the terms under which you offer them.                                             |
//! | `preferred_languages` |    list (optional)   | A comma-separated list of preferred languages (ISO 639-1).                                                                                                                                                                      |
//! | `source_code`         |    link (optional)   | A URL to the project's source code.                                                                                                                                                                                             |
//! | `encryption`          | link/text (optional) | A PGP public key block (or similar) or a link to one.                                                                                                                                                                           |
//! | `auditors`            | link/list (optional) | A comma-separated list of people or entities that audited this smart contract, or a link to a page where audit reports are hosted. Note that this field is self-reported by the author of the program and might not be acurate. |
//! | `acknowledgements`    | link/text (optional) | Either a link or a text document containing acknowledgements to security researchers who have previously found vulnerabilities in the project.                                                                                  |
//! | `expiry`              |    date (optional)   | The date the security.txt will expire. The format is YYYY-MM-DD.                                                                                                                                                                |
//!
//! ## How it works
//! The macro inserts a `&str` into the `.security.txt` section of the resulting ELF. Because of how Rust strings work, this is a tuple of a pointer to the actual string and the length.
//!
//! The string the macro builds begins with the start marker `=======BEGIN SECURITY.TXT V1=======\0`, and ends with the end marker `=======END SECURITY.TXT V1=======\0`. In between is a list of an even amount of strings, delimited by nullbytes. Every two strings form a key-value-pair.


#[cfg(feature = "parser")]
mod parser;
#[cfg(feature = "parser")]
pub use crate::parser::*;

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

