#![feature(test)]

extern crate test;

use hw_hash_bruteforce::process_hash;
use test::Bencher;

fn bench_compute_n_bytes<const N: usize>(b: &mut Bencher) {
    let password = [b'0'; N];
    let mut md5_hex_buf = [0u8; 32];
    let mut final_hex = [0u8; 64];

    b.iter(|| {
        test::black_box(process_hash(
            &password,
            &mut md5_hex_buf,
            &mut final_hex,
        ))
    });
}

#[bench]
fn bench_compute_8_bytes(b: &mut Bencher) {
    bench_compute_n_bytes::<8>(b);
}

#[bench]
fn bench_compute_6_bytes(b: &mut Bencher) {
    bench_compute_n_bytes::<6>(b);
}
