use rless_safe_core::arena::{with_arena, Arena};
fn main() {
    let _escaped = with_arena(|mut owner: Arena<'_, u8>| owner.try_insert(7).unwrap());
}
