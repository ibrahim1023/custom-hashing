pub fn rotate_left(value: u64, shift: u32) -> u64 {
    (value << shift) | (value >> (64 - shift))
}
