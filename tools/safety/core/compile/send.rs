use rless_safe_core::arena::with_arena;
fn main() {
    with_arena(|mut owner| {
        let handle = owner.try_insert(7u8).unwrap();
        std::thread::scope(|scope| { scope.spawn(move || { let _moved = handle; }); });
    });
}
