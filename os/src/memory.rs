pub mod config;
mod heap;
mod address;
mod frame;

pub fn init() {
    heap::init();

    println!("memory initialized");
}
