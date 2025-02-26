#![feature(new_range_api)]

use argh::FromArgs;
use hw_hash_bruteforce::brute::digits::brute_digits;

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

    let total = 10u64.pow(args.length as u32);
    let range = core::range::Range {
        start: 0,
        end: total,
    };

    match args.mode.as_str() {
        "digits" => brute_digits(args.length, range, &args.target_hash),
        _ => {
            eprintln!("Invalid mode.");
        }
    }
}
