use crate::utilities;

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
    pub parent: *const Self,      // 0x0030
}

impl Instance {
    pub unsafe fn get_name(&self) -> String {
        utilities::read_string(self.name)
    }
}
