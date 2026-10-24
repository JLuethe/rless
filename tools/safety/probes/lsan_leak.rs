// Deliberate detector violation. Leak metadata is intentionally not retained.
#[inline(never)]
fn leak() {
    let raw = Box::into_raw(Box::new([7u8; 8192]));
    std::hint::black_box(raw);
}
fn main() {
    leak();
}
