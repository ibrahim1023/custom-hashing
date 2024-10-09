use crate::utils::bitwise_operations::rotate_left;

pub fn custom_hash1(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;

    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = rotate_left(hash.wrapping_mul(0x100000001b3), 13);
    }

    format!("{:x}", hash)
}
