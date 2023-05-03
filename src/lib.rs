pub mod aslr;
pub mod cprint;

use poggers::internal::windows::module;
use poggers::sigscan::SigScan;
use cprint::{Type, get_print};

#[poggers_derive::create_entry]
fn entry() -> Result<(), String> {
    println!("DLL injected!");

    let rbx_module = module::InModule::new("RobloxPlayerBeta.exe").unwrap();
    println!("T1");
    let asl = aslr::ASLR::new(rbx_module.base_address, 0x003a0000); // Maybe add to poggers
    println!("T2");

    let print = get_print(asl.aslr(0x0108b5b0)); // version-08c4cfa3d43c47ef
    print(Type::Error, "Hi");

    //let scan_res = unsafe { rbx_module.scan("48 8B 05 ? ? ? ? 48 83 C4 48 C3", rbx_module.base_address, rbx_module.size).unwrap() };

    Ok(())
}
