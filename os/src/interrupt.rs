mod context;
mod handler;

pub fn init() {
    handler::init();
    println!("mod interrupt initialized");
}
