/* ---------------------------------------------------------------- */

pub mod data_model;

pub use data_model::DataModel;

/* ---------------------------------------------------------------- */

use std::ffi::*;
use std::ptr::NonNull;

#[repr(C)]
pub struct Instance {
    pub vtable: *const usize,     // 0x0000
    pub this: *mut Self,          // 0x0004
    _pad0: [usize; 2],            // 0x0008..0x000C
    pub descriptor: *const usize, // 0x0010
    _pad1: [usize; 4],            // 0x0014..0x0020
    pub name: *const usize,       // 0x0024
    pub children: *const usize,   // 0x0028
    _pad2: [usize; 1],            // 0x002C
    pub parent: *mut Instance,    // 0x0030
}

impl Instance {
    pub unsafe fn get_descriptor(&self) -> String {
        todo!()
    }

    pub unsafe fn get_name(&self) -> String {
        if *(self.name.byte_offset(0x10) as *const usize) < 16 {
            return CStr::from_ptr(self.name as *const c_char)
                .to_string_lossy()
                .to_string();
        }

        CStr::from_ptr(*(self.name as *const *const c_char))
            .to_string_lossy()
            .to_string()
    }

    pub unsafe fn get_children(&self) -> Option<Vec<&'static mut Instance>> {
        let mut children = Vec::new();

        if self.children.is_null() {
            return None;
        }

        let mut child = *(self.children as *const *const usize);
        let end_child = *(self.children.byte_offset(0x04) as *const *const usize);

        while child != end_child {
            let current_child = NonNull::<Instance>::new(*child as *mut _)
                .expect("`Instance` is a null pointer")
                .as_mut();

            children.push(current_child);
            child = child.byte_offset(0x08);
        }

        Some(children)
    }

    // TODO: Actually figure out a sane way to do this.
    pub unsafe fn get_descendants(&self) -> Option<Vec<&'static mut Instance>> {
        if let Some(children) = self.get_children() {
            let mut descendants = Vec::new();

            for child in children.iter() {
                descendants.push(
                    NonNull::<Instance>::new(child.this)
                        .expect("`Instance` is a null pointer")
                        .as_mut(),
                );

                if child.get_descendants().is_none() {
                    continue;
                }

                descendants.append(&mut child.get_descendants().unwrap());
            }

            return Some(descendants);
        }

        None
    }

    // TODO: Optimize this.
    pub unsafe fn get_full_name(&self) -> String {
        let mut parent = self.parent;
        let mut string = self.get_name();

        while !parent.is_null() {
            let current_parent = &*parent;

            string = format!("{}.", current_parent.get_name()) + &string;

            parent = current_parent.parent;
        }

        string
    }

    pub unsafe fn get_parent(&self) -> &'static Instance {
        &*(self.parent as *mut Instance)
    }
}
