pub mod aslr;
pub mod cprint;

use crate::cprint::{CPrint, Type};

#[poggers_derive::create_entry]
fn entry() -> Result<(), String> {
    println!("DLL injected!");

    let module = poggers::internal::windows::module::InModule::new("RobloxPlayerBeta.exe").unwrap();
    let asl = aslr::ASLR::new(module.base_address, 0x80000); // Maybe add to poggers

    let cprint_addr = asl.aslr(0x00d699c0); // version-31b938635c234124
    let cprint = CPrint::new(cprint_addr);

    cprint.print(Type::Warn, "Trolling");

    Ok(())
}
