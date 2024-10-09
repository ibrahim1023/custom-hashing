#[cfg(test)]
mod tests {
    use crate::core::{ custom_hash1, custom_hash2 };

    #[test]
    fn test_custom_hash1() {
        let input = "test string";
        let hash1 = custom_hash1::custom_hash1(input);

        assert_eq!(hash1.len(), 16);
    }

    #[test]
    fn test_custom_hash2() {
        let input = "another test";
        let hash2 = custom_hash2::custom_hash2(input);
        let different_hash = custom_hash2::custom_hash2("another string");

        assert_ne!(hash2, different_hash); // Hashes should differ
    }
}
