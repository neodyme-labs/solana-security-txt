#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "parser")]
extern crate alloc;

#[cfg(feature = "parser")]
mod parser;
#[cfg(feature = "parser")]
pub use crate::parser::*;

/// Constant for the beginning of the security.txt file.
pub const SECURITY_TXT_BEGIN: &str = "=======BEGIN SECURITY.TXT V1=======\0";
/// Constant for the end of the security.txt file.
pub const SECURITY_TXT_END: &str = "=======END SECURITY.TXT V1=======\0";

#[macro_export]
/// Create a static string containing the security.txt file.
macro_rules! security_txt {
    ($($name:ident: $value:expr),*) => {
        #[cfg_attr(target_arch = "bpf", link_section = ".security.txt")]
        #[allow(dead_code)]
        #[no_mangle]
        /// Static string containing the security.txt file.
        pub static SECURITY_TXT: &str = concat! {
            "=======BEGIN SECURITY.TXT V1=======\0",
            $(stringify!($name), "\0", $value, "\0",)*
            "=======END SECURITY.TXT V1=======\0"
        };
    };
}
