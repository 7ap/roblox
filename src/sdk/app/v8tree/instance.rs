use std::ffi::c_char;
use std::ops::Deref;

use crate::sdk::app::reflection::*;

#[repr(C)]
pub struct Instance {
    _super0: DescribedBase,                   // 0x000..0x018
    pub todo_archivable: usize,               // 0x018..0x01C
    pub todo_is_parent_locked: usize,         // 0x01C..0x020
    pub todo_is_setting_parent: usize,        // 0x020..0x024
    pub todo_ancestor_permission_mask: usize, // 0x024..0x028
    pub name: *const c_char,                  // 0x028..0x02C
    pub children: [usize; 2],                 // 0x02C..0x034
    pub parent: *mut Instance,                // 0x034..0x038
}

impl Deref for Instance {
    type Target = DescribedBase;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}
