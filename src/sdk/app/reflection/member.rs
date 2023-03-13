use std::ffi::*;
use std::ops::Deref;

use crate::sdk::app::reflection::*;
use crate::sdk::app::security::*;
use crate::sdk::base::*;
use crate::sdk::extras::*;

#[repr(C)]
pub struct MemberDescriptor {
    _super0: Descriptor,              // 0x000..0x014
    pub category: *const cxx::String, // 0x014..0x018
    pub owner: *mut ClassDescriptor,  // 0x018..0x01C
    pub security: Permissions,        // 0x01C..0x020
}

impl Deref for MemberDescriptor {
    type Target = Descriptor;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

#[repr(C)]
pub struct MemberDescriptorContainer {
    pub descriptors: cxx::Vector<*mut MemberDescriptor>, // 0x000..0x00C
    pub descriptor_lookup: DenseHashMap<*const c_char, *mut MemberDescriptor>, // 0x00C..0x024
    pub derived_containers: cxx::Vector<*mut MemberDescriptorContainer>, // 0x024..0x030
    pub base: *const MemberDescriptorContainer,          // 0x030..0x034
    pub descriptor_added_callback: boost::FunctionBase,  // 0x034..0x05C
}
