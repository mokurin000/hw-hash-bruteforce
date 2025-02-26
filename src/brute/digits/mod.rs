use core::range::Range;

use crate::process_hash;

pub fn brute_digits(length: u8, range: Range<u64>, target_hash: &str) {
    let mut password = [0u8; u8::MAX as usize];
    let mut hex_buf = [0u8; 32];
    let mut final_hex = [0u8; 64];

    for num in range {
        let mut n = num;
        for j in (0..length).rev() {
            password[j as usize] = b'0' + (n % 10) as u8;
            n /= 10;
        }

        process_hash(&password[0..length as usize], &mut hex_buf, &mut final_hex);

        if final_hex == target_hash.as_bytes() {
            println!("Found password: {}", String::from_utf8_lossy(&password));
            return;
        }
    }
}
