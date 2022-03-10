# security.txt

[![](https://img.shields.io/crates/v/solana-security-txt)](https://crates.io/crates/solana-security-txt) [![](https://docs.rs/solana-security-txt/badge.svg)](https://docs.rs/solana-security-txt/) [![](https://img.shields.io/crates/v/query-security-txt)](https://crates.io/crates/query-security-txt)

This library defines a macro, whose aim it is to provide easy-to-parse information to security researchers that wish to contact the authors of a Solana smart contract.
It is inspired by https://securitytxt.org/.

Add the following to the `[dependencies]` section of your Cargo.toml:
```toml
solana-security-txt = "0.1.1"
```

To install the querying tool, execute
```
cargo install query-security-txt
```


## Example
```rust
security_txt! {
    name: "Example",
    project_url: "http://example.com",
    source_code: "https://github.com/example/example",
    expiry: "2042-01-01",
    preferred_languages: "en,de",
    contacts: "email:example@example.com,discord:example#1234",
    encryption: "
-----BEGIN PGP PUBLIC KEY BLOCK-----
Comment: Alice's OpenPGP certificate
Comment: https://www.ietf.org/id/draft-bre-openpgp-samples-01.html

mDMEXEcE6RYJKwYBBAHaRw8BAQdArjWwk3FAqyiFbFBKT4TzXcVBqPTB3gmzlC/U
b7O1u120JkFsaWNlIExvdmVsYWNlIDxhbGljZUBvcGVucGdwLmV4YW1wbGU+iJAE
ExYIADgCGwMFCwkIBwIGFQoJCAsCBBYCAwECHgECF4AWIQTrhbtfozp14V6UTmPy
MVUMT0fjjgUCXaWfOgAKCRDyMVUMT0fjjukrAPoDnHBSogOmsHOsd9qGsiZpgRnO
dypvbm+QtXZqth9rvwD9HcDC0tC+PHAsO7OTh1S1TC9RiJsvawAfCPaQZoed8gK4
OARcRwTpEgorBgEEAZdVAQUBAQdAQv8GIa2rSTzgqbXCpDDYMiKRVitCsy203x3s
E9+eviIDAQgHiHgEGBYIACAWIQTrhbtfozp14V6UTmPyMVUMT0fjjgUCXEcE6QIb
DAAKCRDyMVUMT0fjjlnQAQDFHUs6TIcxrNTtEZFjUFm1M0PJ1Dng/cDW4xN80fsn
0QEA22Kr7VkCjeAEC08VSTeV+QFsmz55/lntWkwYWhmvOgE=
=iIGO
-----END PGP PUBLIC KEY BLOCK-----
",
    acknowledgements: "
The following hackers could've stolen all our money but didn't:
- Neodyme
",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md"
}
```

## Format
All values need to be string literals that may not contain nullbytes.
Naive parsers may fail if the binary contains one of the security.txt delimiters anywhere else
(`=======BEGIN SECURITY.TXT V1=======\0` and `=======END SECURITY.TXT V1=======\0`).

The following fields are supported, some of which are required for this to be considered a valid security.txt:
- `name` (required): The name of the project.
-  `project_url` (required): A URL to the project's homepage/dapp.
- `source_code` (optional): A URL to the project's source code.
- `expiry` (optional): The date the security.txt will expire. The format is YYYY-MM-DD.
- `preferred_languages` (required): A comma-separated list of preferred languages.
- `contacts` (required): A comma-separated list of contact information in the format `<contact type>:<contact information>`. Possible contact types are `email`, `discord`, `telegram`, `twitter`, `link` and `other`.
- `encryption` (optional): A PGP public key block (or similar) or a link to one
- `acknowledgements` (optional): Either a link or a Markdown document containing acknowledgements to security researchers that have found vulnerabilities in the project in the past.
- `policy` (required): Either a link or a Markdown document describing the project's security policy. This should describe what kind of bounties your project offers and the terms under which you offer them.

## How it works
The macro inserts a `&str` into the `.security.txt` section of the resulting ELF. Because of how Rust strings work, this is a tuple of a pointer to the actual string and the length.

The string the macro builds begins with the start marker `=======BEGIN SECURITY.TXT V1=======\0`, and ends with the end marker `=======END SECURITY.TXT V1=======\0`. In between is a list of an even amount of strings, delimited by nullbytes. Every two strings form a key-value-pair.