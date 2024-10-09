pub fn pad_message(message: &str) -> Vec<u8> {
    let mut padded = message.as_bytes().to_vec();
    let len = message.len();

    // Add padding byte 0x80
    padded.push(0x80);

    while padded.len() % 64 != 56 {
        padded.push(0x00);
    }

    // Append original length of the message (in bits)
    padded.extend_from_slice(&((len as u64) * 8).to_be_bytes());

    padded
}
