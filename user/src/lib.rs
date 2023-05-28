#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;

mod lang_items;
mod syscall;
mod logging;

use log::{info, debug, trace};

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    logging::init();
    info!("Enter user/src/lib.rs _start() ");
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

// need to add at the top: #![feature(linkage)]
#[linkage = "weak"]  
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}


use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    logging::init();
    debug!("Enter user/src/lib.rs exit() ");
    sys_exit(exit_code)
}
