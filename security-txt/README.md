# security.txt

[![](https://img.shields.io/crates/v/solana-security-txt)](https://crates.io/crates/solana-security-txt) [![](https://docs.rs/solana-security-txt/badge.svg)](https://docs.rs/solana-security-txt/) 

[![](https://img.shields.io/crates/v/query-security-txt)](https://crates.io/crates/query-security-txt)

This library defines a macro, which allows developers to provide easy-to-parse information to security researchers that wish to contact the authors of a Solana smart contract.
It is inspired by <https://securitytxt.org/>.

See this example in the Solana Explorer: <https://explorer.solana.com/address/HPxKXnBN4vJ8RjpdqDCU7gvNQHeeyGnSviYTJ4fBrDt4/security?cluster=devnet>


## Motivation

Users typically interact with a Solana smart contract via the project's web interface, which knows the contract's address. Security researchers often don't.

Especially for smaller or private projects, identification from just the contract's address is hard and time-consuming, if not impossible.
This slows down or prevents bug reports from reaching the developers.

Having standardized information about your project inside your contract makes it easy for whitehat researchers to reach you if they find any problems.

To maximize compatibility with existing deployment setups, multisigs and DAOs, this security.txt is implemented to simply be a part of your program rather than an external contract.

## Usage

Add the following to the `[dependencies]` section of your Cargo.toml:
```toml
solana-security-txt = "1.1.2"
```

To install the querying tool, execute
```
cargo install query-security-txt
```

In general, there are two ways to specify the information. Either directly inside the contract to store it on-chain or by linking to a web page. The former has the advantage that it is easy to set up, but has the drawback that any changes require a program upgrade. Program upgrades shouldn't be done lightly.

Therefore, **it is recommended to have all information you expect to change on a website, which you can then link to inside the security.txt**.

As many projects are best reachable via Telegram or Discord there is native support for these contact methods. But be aware that handles might change, for example, if you change your Discord username.

The `security_txt` macro is intentionally kept brief. As such, it doesn't do any input validation. For optimal experience, **please verify the format before uploading the contract to the chain.** This can be done with the provided `query-security-txt` program, which can not only be called with on-chain contracts but also local binaries:

```sh
query-security-txt target/sbpf-solana-solana/release/example_contract.so
```

#### Notice for library authors
If you expect your contract to be used as a dependency in other contracts, you **must** exclude the macro when your
contract is being built as a library, i.e., when the `no-entrypoint` feature is being used.
Consult the example snippet below or the full example in the `example-contract` directory for details.

#### Troubleshooting: linker error `multiple definition of security_txt`
If you encounter this error during building, then that means that the `security_txt` macro has been used multiple times.
This is probably caused by one of your dependencies also using the macro, which causes a name conflict during building.

In that case, please tell the authors of that dependency to read the above notice for library authors and add the
following to the macro to exclude it from `no-entrypoint` builds.
```rust
#[cfg(not(feature = "no-entrypoint"))]
```

#### Use as an indicator for the deployed code version

In order to simplify access to the source code we recommend to include the commit hash as `source_revision` or the release tag as `source_release`.
You can use the `env!` macro to automatically configure values passed to the `security_txt!` macro from the build process envioronment.

### Example

```rust
#[cfg(not(feature = "no-entrypoint"))]
use {default_env::default_env, solana_security_txt::security_txt};

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Example",
    project_url: "http://example.com",
    contacts: "email:example@example.com,link:https://example.com/security,discord:example#1234",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md",

    // Optional Fields
    preferred_languages: "en,de",
    source_code: "https://github.com/example/example",
    source_revision: default_env!("GITHUB_SHA", ""),
    source_release: default_env!("GITHUB_REF_NAME", ""),
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
    auditors: "None",
    acknowledgements: "
The following hackers could've stolen all our money but didn't:
- Neodyme
"
}
```

### Example Policies

Bug bounty policies can be a bit daunting to write. For a good and thorough example, take a look at Solana Foundation's [SECURITY.md](https://github.com/solana-labs/solana/blob/master/SECURITY.md). But even a very brief policy is better than none. A starting point might be something like:

> We pay a bug bounty at our discretion after verifying the bug, up to 10% of value at risk, limited by a maximum of $x. This bounty is only paid out if details about the security issues have not been provided to third parties before a fix has been introduced and verified. Furthermore, the reporter is in no way allowed to exploit the issue without our explicit consent.

If you don't pay bounties, which might be sensible for toy projects that don't handle much value, you can also put something like

> We do not pay a bug bounty.

For more inspiration, take a look at how other large Solana projects structure their policies (random, non-exhaustive collection):
- <https://github.com/solana-labs/solana/security/policy>
- <https://forum.projectserum.com/t/formalizing-a-bug-bounty-program/410>
- <https://docs.marinade.finance/developers/bug-bounty>
- <https://docs.solend.fi/protocol/bug-bounty>
- <https://github.com/certusone/wormhole/blob/dev.v2/ImmuneFi%20bug-bounty.md>
- <https://immunefi.com/bounty/lido/ >
- <https://docs.mango.markets/mango/bug-bounty>


## Format

This crate uses a macro to construct one long security.txt string. It begins with the start marker `=======BEGIN SECURITY.TXT V1=======\0`, and ends with the end marker `=======END SECURITY.TXT V1=======\0`.
In between is a list of strings, delimited by nullbytes. Every pair of two strings forms a key-value pair.

All values need to be string literals that may not contain nullbytes.
The contract should not include the security.txt markers anywhere else, otherwise naive parsers might fail.

The following fields are supported, some of which are required for this to be considered a valid security.txt:

| Field                 |         Type         | Description                                                                                                                                                                                                                                                                                                                                    |
| --------------------- | :------------------: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`name`**            |  string (required)   | The name of the project. If the project isn't public, you can put `private`.                                                                                                                                                                                                                                                                   |
| **`project_url`**     | https url (required) | A URL to the project's homepage/dapp. If the project isn't public, you can put `private`.                                                                                                                                                                                                                                                      |
| **`contacts`**        |   list (required)    | A comma-separated list of contact information in the format `<contact type>:<contact information>`. Should roughly be ordered in preference. Possible contact types are `email`, `link`, `discord`, `telegram`, `twitter` and `other`. Prefer contact types that likely won't change for a while, like a `security@example.com` email address. |
| **`policy`**          | link/text (required) | Either a link or a text document describing the project's security policy. This should describe what kind of bounties your project offers and the terms under which you offer them.                                                                                                                                                            |
| `preferred_languages` |   list (optional)    | A comma-separated list of preferred languages (ISO 639-1).                                                                                                                                                                                                                                                                                     |
| `encryption`          | link/text (optional) | A PGP public key block (or similar) or a link to one.                                                                                                                                                                                                                                                                                          |
| `source_code`         |   link (optional)    | A URL to the project's source code.                                                                                                                                                                                                                                                                                                            |
| `source_release`      |   string (optional   | The release identifier of this build, ideally corresponding to a tag on git that can be rebuilt to reproduce the same binary. 3rd party build verification tools will use this tag to identify a matching github releases.                                                                                                                     |
| `source_revision`     |  string (optional)   | The revision identifier of this build, usually a git sha that can be rebuilt to reproduce the same binary. 3rd party build verification tools will use this tag to identify a matching github releases.                                                                                                                                        |
| `auditors`            | link/list (optional) | A comma-separated list of people or entities that audited this smart contract, or a link to a page where audit reports are hosted. Note that this field is self-reported by the author of the program and might not be accurate.                                                                                                               |
| `acknowledgements`    | link/text (optional) | Either a link or a text document containing acknowledgements to security researchers who have previously found vulnerabilities in the project.                                                                                                                                                                                                 |
| `expiry`              |   date (optional)    | The date the security.txt will expire. The format is YYYY-MM-DD.                                                                                                                                                                                                                                                                               |

## Security of this Crate
To minimize dependencies, the security.txt parser is disabled by default, and will only be built if the feature `parser` is set.

Literally all this crate does is define a single macro:

```rust
#[macro_export]
macro_rules! security_txt {
    ($($name:ident: $value:expr),*) => {
        #[cfg_attr(target_arch = "bpf", link_section = ".security.txt")]
        #[allow(dead_code)]
        #[no_mangle]
        pub static security_txt: &str = concat! {
            "=======BEGIN SECURITY.TXT V1=======\0",
            $(stringify!($name), "\0", $value, "\0",)*
            "=======END SECURITY.TXT V1=======\0"
        };
    };
}
```

If you want, you can just copy this into your contract instead of depending on this crate.

Should you notice any errors, please don't hesitate to open an issue, or in critical cases reach us under `contact@neodyme.io`.

### Additional ELF Section

In addition to inserting the security.txt string into the binary, the macro creates a new `.security.txt` ELF section via the `#[link_section]` attribute. Because of how Rust strings work, it is not easily possible to place the entire string in a separate ELF section, so this is simply a tuple of a pointer to the actual string and its length.

ELF-aware parsers can thus simply look at this section and are not required to search the haystack for the security.txt markers.

Since Solana may move away from ELF binaries in the future, this section is optional in the standard.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
