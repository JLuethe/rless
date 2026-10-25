use rless_safe_core::arena::with_arena;
use rless_safe_core::bytebuf::ByteBuf;
use rless_safe_core::coordinate::{RetainedRange, SourceOffset};

fn main() {
    let mut bytes = ByteBuf::try_from_slice(b"abc").unwrap();
    bytes.try_extend_from_within(0..2).unwrap();
    assert_eq!(bytes.as_bytes(), b"abcab");
    with_arena(|mut owner| {
        let handle = owner.try_insert(7u8).unwrap();
        assert_eq!(owner.get(handle), Some(&7));
        assert_eq!(owner.remove(handle), Some(7));
        assert_eq!(owner.get(handle), None);
    });
    let range = RetainedRange::new(SourceOffset::new(1 << 33).unwrap(), 3).unwrap();
    assert_eq!(range.index(range.start()).unwrap().get(), 0);
}
