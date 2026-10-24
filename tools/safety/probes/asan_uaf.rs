// Deliberate detector violation. This crate is outside the production workspace.
fn main() {
    let owned = Box::new(41u64);
    let raw = Box::into_raw(owned);
    unsafe {
        drop(Box::from_raw(raw));
        std::hint::black_box(std::ptr::read_volatile(raw));
    }
}
