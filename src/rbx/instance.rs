pub mod data_model;

use crate::utilities;

pub use data_model::DataModel;

#[repr(C)]
pub struct Instance {
    pub vtable: *const usize,     // 0x0000
    pub this: *const Self,        // 0x0004
    _pad0: [usize; 2],            // 0x0008..0x000C
    pub descriptor: *const usize, // 0x0010
    _pad1: [usize; 4],            // 0x0014..0x0020
    pub name: *const usize,       // 0x0024
    pub children: *const usize,   // 0x0028
    _pad2: [usize; 1],            // 0x002C
    pub parent: *const Instance,  // 0x0030
}

impl Instance {
    pub unsafe fn get_descriptor(&self) -> String {
        todo!()
    }

    pub unsafe fn get_name(&self) -> String {
        utilities::read_string(self.name)
    }

    pub unsafe fn get_children(&self) -> Vec<&'static Instance> {
        let mut children = Vec::new();

        let mut child = *(self.children as *const *const usize);
        let end_child = *(self.children.byte_offset(0x04) as *const *const usize);

        while child != end_child {
            children.push(&*(*child as *mut Instance));
            child = child.byte_offset(0x08);
        }

        children
    }

    pub unsafe fn get_parent(&self) -> &'static Instance {
        &*(self.parent as *mut Instance)
    }
}
