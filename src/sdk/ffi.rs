//! Yes, I'm aware the entirety of `sdk` is *technically* FFI bindings - I don't
//! care.
//!
//! This is supposed to house the more *miscellaneous* bindings (not in `RBX`
//! namespace)

mod cxx_string;

pub use cxx_string::CxxString;
