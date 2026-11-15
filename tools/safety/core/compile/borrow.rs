use rless_safe_core::bytebuf::ByteBuf;
fn main() {
    let mut bytes = ByteBuf::try_from_slice(b"old").unwrap();
    let retained = bytes.as_bytes();
    bytes.try_extend(b"new").unwrap();
    assert_eq!(retained, b"old");
}
