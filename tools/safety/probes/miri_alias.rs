// Deliberate alias violation; keep Miri's normal provenance/alias checks enabled.
fn main() {
    let mut value = 0;
    let first = &mut value;
    let raw = first as *mut i32;
    unsafe {
        let second = &mut *raw;
        *first = 1;
        *second = 2;
    }
}
