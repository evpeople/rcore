#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_items;
mod sbi;
use  core::arch::global_asm;

global_asm!(include_str!("entry.asm"));


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello rcore");
    panic!("Shutdown machine!");
    loop {}
}
fn clear_bss(){
    extern "C"{
        fn sbss();
        fn ebss();
    }
    //first find sbss and ebss ,then zero them,it translate to uszie
    ( sbss as usize..ebss as usize).for_each(|a|{
        unsafe{(a as *mut u8).write_volatile(0)}
    });
}
// fn main() {
// //    println!("Hello, world!");
// }
