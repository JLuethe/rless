use rless_safe_core::arena::{with_arena, Arena};
fn main() {
    with_arena(|mut first: Arena<'_, u8>| {
        let handle = first.try_insert(7).unwrap();
        with_arena(|second: Arena<'_, u8>| { second.get(handle); });
    });
}
