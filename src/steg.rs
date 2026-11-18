use image::RgbaImage;
use std::error::Error;

pub fn encode(img: &mut RgbaImage, message: &[u8], lsb_depth: u8) -> Result<(), Box<dyn Error>> {
    if lsb_depth == 0 || lsb_depth > 8 {
        return Err("LSB depth must be between 1 and 8".into());
    }

    let bytes: &mut [u8] = img.as_mut();

    let bits_per_byte = lsb_depth as usize; //depth of LSB encoding
    let length = message.len() as u32; //message length header for decoding purposes, stored in first 32 bits

    let min_size_bits = 32 + message.len() * 8;
    let required_bytes = (min_size_bits + bits_per_byte - 1) / bits_per_byte;

    if required_bytes > bytes.len() {
        return Err("image is too small for the payload".into());
    }

    let mut bits: Vec<u8> = Vec::with_capacity(min_size_bits);

    for i in (0..32).rev() {
        bits.push(((length >> i) & 1) as u8); //push length header into first 32 bits
    }

    for byte in message {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1); //push message into bits Vec<>, starting with MSB
        }
    }

    // Mask for the selected LSB depth
    let mask = (1u8 << lsb_depth) - 1;

    // Write multiple bits per image byte
    for (i, chunk) in bits.chunks(bits_per_byte).enumerate() {
        let mut value = 0u8;

        for bit in chunk {
            value = (value << 1) | bit;
        }

        // Align bits to the least significant positions
        value <<= bits_per_byte - chunk.len();

        bytes[i] = (bytes[i] & !mask) | value;
    }

    Ok(())
}

pub fn decode(img: &RgbaImage, lsb_depth: u8) -> Result<Vec<u8>, Box<dyn Error>> {
    if lsb_depth == 0 || lsb_depth > 8 {
        return Err("LSB depth must be between 1 and 8".into());
    }

    let bytes: &[u8] = img.as_ref();
    let bits_per_byte = lsb_depth as usize;
    let mask = (1u8 << lsb_depth) - 1;

    let min_header_bytes = (32 + bits_per_byte - 1) / bits_per_byte;

    if bytes.len() < min_header_bytes {
        return Err("image is too small to be decoded".into());
    }

    // Extract the 32-bit length header
    let mut length = 0u32;

    for i in 0..32 {
        let byte_index = i / bits_per_byte;
        let bit_index = i % bits_per_byte;

        let extracted = (bytes[byte_index] & mask)
            >> (bits_per_byte - 1 - bit_index);

        length = (length << 1) | extracted as u32;
    }

    // Calculate required number of bits and image bytes
    let min_size_bits = 32 + length as usize * 8;
    let required_bytes =
        (min_size_bits + bits_per_byte - 1) / bits_per_byte;

    if required_bytes > bytes.len() {
        return Err("invalid message length".into());
    }

    let mut message = Vec::with_capacity(length as usize);

    // Extract message bits
    for i in 0..length as usize {
        let mut byte = 0u8;

        for j in 0..8 {
            let bit_position = 32 + i * 8 + j;

            let byte_index = bit_position / bits_per_byte;
            let bit_index = bit_position % bits_per_byte;

            let extracted = (bytes[byte_index] & mask)
                >> (bits_per_byte - 1 - bit_index);

            byte = (byte << 1) | extracted;
        }

        message.push(byte);
    }

    Ok(message)
}
