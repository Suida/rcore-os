#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod interrupt;
mod panic;
mod sbi;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();
    println!("Hello rCore-Tutorial!");

    unsafe {
        llvm_asm!("ebreak" :::: "volatile");
    };

    unreachable!()
}
