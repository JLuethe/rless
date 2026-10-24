#![forbid(unsafe_code)]
fn main() {
    let mut bytes = vec![0u8, 1, 255];
    bytes.try_reserve(1024).unwrap();
    bytes.extend_from_slice(&[2, 3, 4]);
    assert_eq!(bytes.len(), 6);
    assert_eq!(bytes[2], 255);
}
