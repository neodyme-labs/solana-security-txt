# security.txt

[![](https://img.shields.io/crates/v/solana-security-txt)](https://crates.io/crates/solana-security-txt) [![](https://docs.rs/solana-security-txt/badge.svg)](https://docs.rs/solana-security-txt/) 

[![](https://img.shields.io/crates/v/query-security-txt)](https://crates.io/crates/query-security-txt)

This library defines a macro, which allows developers to provide easy-to-parse information to security researchers that wish to contact the authors of a Solana smart contract.
It is inspired by https://securitytxt.org/.

## Motivation

Users typically interact with a Solana smart contract via the project's web interface, which knows the contract's address. Security researchers often don't.

Especially for smaller projects, identifying a project from just the contract's address is hard and time-consuming, if not impossible.
This slows down or prevents bug reports from reaching the developers.

Having standardized information about your project inside your contract makes it easy for whitehat researchers to reach you if they find any problems.


## Format

This crates uses a macro to construct one long security.txt string. It begins with the start marker `=======BEGIN SECURITY.TXT V1=======\0`, and ends with the end marker `=======END SECURITY.TXT V1=======\0`.
In between is a list of strings, delimited by nullbytes. Every pair of two strings forms a key-value pair.

All values need to be string literals that may not contain nullbytes.

The following fields are supported, some of which are required for this to be considered a valid security.txt:
- `name` (required): The name of the project.
- `project_url` (required): A URL to the project's homepage/dapp.
- `source_code` (optional): A URL to the project's source code.
- `expiry` (optional): The date the security.txt will expire. The format is YYYY-MM-DD.
- `preferred_languages` (required): A comma-separated list of preferred languages.
- `contacts` (required): A comma-separated list of contact information in the format `<contact type>:<contact information>`. Possible contact types are `email`, `discord`, `telegram`, `twitter`, `link` and `other`.
- `auditors` (optional): A comma-separated list of people or entities that audited this smart contract. Note that this field is self-reported by the author of the program and might not be acurate.
- `encryption` (optional): A PGP public key block (or similar) or a link to one
- `acknowledgments` (optional): Either a link or a text document containing acknowledgments to security researchers who have previously found vulnerabilities in the project.
- `policy` (required): Either a link or a text document describing the project's security policy. This should describe what kind of bounties your project offers and the terms under which you offer them.

Naive parsers may fail if the binary contains one of the security.txt markers anywhere else.

## Usage

Add the following to the `[dependencies]` section of your Cargo.toml:
```toml
solana-security-txt = "0.1.4"
```

To install the querying tool, execute
```
cargo install query-security-txt
```

In general, there are two ways to specify the information. Either directly inside the contract to store it on-chain or by linking to a web page.
The former has the advantage that it is easy to set up but has the drawback that any changes require a program upgrade. Program upgrades shouldn't be done lightly.

Therefore it is recommended to link to all relevant information that might change instead of hardcoding it on-chain.

As many projects are best reachable via Telegram or Discord there is native support for these contact methods. But be aware that handles might change, for example, if you change your Discord username. 

### Example
```rust
security_txt! {
    name: "Example",
    project_url: "http://example.com",
    source_code: "https://github.com/example/example",
    expiry: "2042-01-01",
    preferred_languages: "en,de",
    contacts: "email:example@example.com,discord:example#1234",
    auditors: "Neodyme",
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
    acknowledgments: "
The following hackers could've stolen all our money but didn't:
- Neodyme
",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md"
}
```


## Additional ELF Section

In addition to inserting the security.txt string into the binary, the macro creates a new `.security.txt` ELF section. Because of how Rust strings work, it is not easily possible to place the entire string in a separate ELF section, so this is simply a tuple of a pointer to the actual string and its length.

ELF-aware parsers can thus simply look at this section and are not required to search the haystack for the security.txt markers.

Since Solana may move away from ELF binaries in the future, this section is optional in the standard.