use std::ffi::*;

use crate::sdk::extras::*;

#[repr(C)]
pub struct DenseHashMap<Key, Value> {
    pub data: cxx::Vector<cxx::Pair<Key, Value>>, // 0x000..0x00C
    pub count: isize,                             // 0x00C..0x010
    pub empty_key: *const c_char,                 // 0x010..0x014
    pub hasher: [c_char; 2],                      // 0x014..0x016
    pub eq: [c_char; 2],                          // 0x016..0x018
}
