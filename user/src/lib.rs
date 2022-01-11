#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

#[no_mangle]
#[link_section = ".text.entry"]
//clear data and exec main,named the section .text.entry
pub extern "C" fn _start() -> ! {
    clear_bss();
    //the exit is provided by the bottom func, use syscall
    //the main is provided by follow, but it link is weak,that means the main actually provided by bin/*.rs,which use lib.rs,and *.rs used no_mangle to  make sure the  name after compile is main
    //but the exit is and only is the exit fun provided by follow
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    //when the func is exec,that mean the *.rs used the lib,don't have a main func
    panic!("Cannot find main!");
}

fn clear_bss() {
    //import C , use write_volatile(0)
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| {
        unsafe { (addr as *mut u8).write_volatile(0); }
    });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }