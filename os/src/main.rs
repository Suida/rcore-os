#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod interrupt;
mod panic;
mod sbi;
mod memory;

#[macro_use]
extern crate lazy_static;
extern crate alloc;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();
    memory::init();

    use alloc::boxed::Box;
    use alloc::vec::Vec;

    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..1_0000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 1_0000);
    for (i, v) in vec.into_iter().enumerate() {
        assert_eq!(i, v);
    }
    println!("heap test passed");

    println!("{}", *memory::config::KERNEL_END_ADDRESS);

    for _ in 0..2 {
        let frame_0 = memory::FRAME_ALLOCATOR.lock()
                                             .alloc()
                                             .unwrap_or_else(|err| panic!("{}", err));
        let frame_1 = memory::FRAME_ALLOCATOR.lock()
                                             .alloc()
                                             .unwrap_or_else(|err| panic!("{}", err));
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

    panic!()
}
