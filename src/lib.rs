#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "pre-generated-bindings"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// To ensure openssl is linked
#[cfg(feature = "enable-openssl")]
#[doc(hidden)]
pub fn __ensure_openssl_linked() {
    let _f = openssl_sys::DTLS_method;
}

#[cfg(feature = "pre-generated-bindings")]
mod srtp;

#[cfg(feature = "pre-generated-bindings")]
pub use srtp::*;
