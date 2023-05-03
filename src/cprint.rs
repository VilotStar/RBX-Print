use std::{ffi::{c_char, CString}, mem::transmute};

#[repr(u8)]
pub enum Type {
    Log,
    Info,
    Warn,
    Error
}

pub fn get_print(addr: usize) -> impl Fn(Type, &str) -> () {
    let cprint_addr = addr as *const ();
    let cprint = unsafe { transmute::<*const (), extern "C" fn (Type, *const c_char, ...) -> ()>(cprint_addr) };

    move |msg_type, msg| {
        let c_str = CString::new(msg).unwrap();
        (cprint)(msg_type, c_str.as_ptr());
    }
}
