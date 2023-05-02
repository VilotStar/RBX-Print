use std::{ffi::{c_char, CString}, mem::transmute};

#[repr(u8)]
pub enum Type {
    Log,
    Info,
    Warn,
    Error
}

pub struct CPrint {
    cprint: extern "C" fn (Type, *const c_char, ...)
}

impl CPrint {
    pub fn new(addr: usize) -> Self {
        let cprint_addr = addr as *const ();
        let cprint = unsafe { transmute::<*const (), extern "C" fn (Type, *const c_char, ...) -> ()>(cprint_addr) };

        Self {
            cprint
        }
    }

    pub fn print(&self, msg_type: Type, msg: &str) {
        let c_str = CString::new(msg).unwrap();
        (self.cprint)(msg_type, c_str.as_ptr());
    }
}
