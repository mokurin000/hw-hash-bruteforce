use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use argh::FromArgs;
use hw_hash_bruteforce::process_hash;

#[derive(FromArgs, Clone)]
/// Brute-force password cracker
struct BruteForceArgs {
    /// mode: "digits" or "ascii"
    #[argh(option)]
    mode: String,

    /// password length
    #[argh(option)]
    length: u8,

    /// target hash to match
    #[argh(positional)]
    target_hash: String,
}

fn main() {
    let args: BruteForceArgs = argh::from_env();

    let found = Arc::new(AtomicBool::new(false));

    match args.mode.as_str() {
        "digits" => brute_digits(&args, found.clone()),
        _ => {
            eprintln!("Invalid mode.");
        }
    }
}

fn brute_digits(args: &BruteForceArgs, found: Arc<AtomicBool>) {
    let total = 10u64.pow(args.length as u32);

    let mut password = vec![b'0'; args.length as usize];
    let mut hex_buf = [0u8; 32];
    let mut final_hex = [0u8; 64];

    (0..total).for_each(|num| {
        if found.load(Ordering::Relaxed) {
            return;
        }

        let mut n = num;
        for j in (0..args.length).rev() {
            password[j as usize] = b'0' + (n % 10) as u8;
            n /= 10;
        }

        process_hash(&password, &mut hex_buf, &mut final_hex);

        if final_hex == args.target_hash.as_bytes() {
            println!("Found password: {}", String::from_utf8_lossy(&password));
            found.store(true, Ordering::Relaxed);
            return;
        }
    });
}
