#![forbid(unsafe_code)]
fn main() {
    unsafe { std::ptr::read_volatile(&0u8); }
}
