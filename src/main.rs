#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_items;
mod sbi;
use core::arch::global_asm;
use log::{error, info, trace, warn, LevelFilter};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    console::init();
    log::set_max_level(LevelFilter::Error);
    error!("sad");
    warn!("21432");
    println!("hello rcorefdsfds");

    info!("123345ddsadsaasdasdas");

    panic!("Shutdown machine!");
    loop {}
}
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    //first find sbss and ebss ,then zero them,it translate to uszie
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
