use std::ops::Deref;

use crate::sdk::app::reflection::*;
use crate::sdk::app::util::*;
use crate::sdk::extras::*;

pub type Instances = cxx::Vector<boost::SharedPtr<Instance>>;

#[repr(C)]
pub struct Instance {
    _super0: DescribedBase,                   // 0x000..0x018
    pub todo_archivable: usize,               // 0x018..0x01C
    pub todo_is_parent_locked: usize,         // 0x01C..0x020
    pub todo_is_setting_parent: usize,        // 0x020..0x024
    pub todo_ancestor_permission_mask: usize, // 0x024..0x028
    pub name: *const cxx::String,             // 0x028..0x02C
    pub children: CopyOnWritePtr<Instances>,  // 0x02C..0x034
    pub parent: *mut Instance,                // 0x034..0x038
}

impl Deref for Instance {
    type Target = DescribedBase;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

impl Instance {
    // TODO: Restructure the whole std::vector -> std::vec::Vec implementation - this is messy and inefficient.
    pub fn get_children(&self) -> Vec<*mut Instance> {
        let mut buffer = Vec::new();
        let children = unsafe { &mut *self.children.object.px }.convert();

        for child in children {
            let instance = unsafe { (*child).px };

            buffer.push(instance);
        }

        buffer
    }
}
