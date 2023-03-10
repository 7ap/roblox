use std::ops::Deref;

use crate::sdk::app::reflection::*;
use crate::sdk::extras::*;

#[repr(C)]
pub struct ClassDescriptor {
    _super0: Descriptor,                                       // 0x000..0x018
    pub property_descriptors: MemberDescriptorContainer,       // 0x018..0x078
    pub event_descriptors: MemberDescriptorContainer,          // 0x078..0x0D8
    pub function_descriptors: MemberDescriptorContainer,       // 0x0D8..0x138
    pub yield_function_descriptors: MemberDescriptorContainer, // 0x138..0x198
    pub callback_descriptors: MemberDescriptorContainer,       // 0x198..0x1F8
}

impl Deref for ClassDescriptor {
    type Target = Descriptor;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

#[repr(C)]
pub struct DescribedBase {
    _super0: EventSource,                 // 0x000..0x004
    _super1: boost::SharedPtr<Self>,      // 0x004..0x00C
    pub descriptor: *mut ClassDescriptor, // 0x00C..0x010
    _todo_xml_id: [usize; 2],             // 0x010..0x018
}
