extern crate cgmath;

mod world;
mod physics;
mod types;

#[no_mangle]
pub extern fn add_one(a: u32) -> u32 {
    a + 1
}
