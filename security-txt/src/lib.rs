#![doc = include_str!("../README.md")]

#[cfg(feature = "parser")]
mod parser;
#[cfg(feature = "parser")]
pub use crate::parser::*;

pub const SECURITY_TXT_BEGIN: &str = "=======BEGIN SECURITY.TXT V1=======\0";
pub const SECURITY_TXT_END: &str = "=======END SECURITY.TXT V1=======\0";

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
