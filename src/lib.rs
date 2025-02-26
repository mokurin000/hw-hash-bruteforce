#![feature(new_range_api)]

use hex_simd::Out;
use sha2::{Digest, Sha256};

pub fn process_hash(password: &[u8], md5_hex_buf: &mut [u8; 32], final_hex: &mut [u8; 64]) {
    // MD5
    let md5_hash = md5::compute(password);

    // Hex encode MD5
    let _ = hex_simd::encode(
        &md5_hash.0,
        Out::from_slice(md5_hex_buf),
        hex_simd::AsciiCase::Lower,
    );

    // SHA256
    let mut hasher = Sha256::new();
    hasher.update(md5_hex_buf);

    let out = Out::from_slice(final_hex);
    let sha_hash = hasher.finalize();

    // Hex encode SHA256
    let _ = hex_simd::encode(&sha_hash, out, hex_simd::AsciiCase::Lower);
}

pub mod brute;
