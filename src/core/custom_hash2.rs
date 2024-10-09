pub fn custom_hash2(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis

    for byte in input.bytes() {
        // Wrapping multiplication by large prime and XOR for better distribution
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= byte as u64;
    }

    format!("{:x}", hash)
}
