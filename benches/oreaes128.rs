use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use ore::{
    ORECipher,
    OREEncrypt,
    scheme::bit2::{
        OREAES128,
        OREAES128CipherText
    }
};

#[inline]
fn do_encrypt_64(input: u64, ore: &mut OREAES128) {
  input.encrypt(ore).unwrap();
}

#[inline]
fn do_encrypt_left_64(input: u64, ore: &mut OREAES128) {
  input.encrypt_left(ore).unwrap();
}

#[inline]
fn do_compare<const N: usize>(a: &OREAES128CipherText<N>, b: &OREAES128CipherText<N>) {
    a.partial_cmp(b);
}

#[inline]
fn do_encrypt_32(input: u32, ore: &mut OREAES128) {
  input.encrypt(ore).unwrap();
}

#[inline]
fn do_encrypt_left_32(input: u32, ore: &mut OREAES128) {
  input.encrypt_left(ore).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let k1 = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    let k2 = hex!("d0d007a5 3f9a6848 83bc1f21 0f6595a3");
    let seed = hex!("d0d007a5 3f9a6848");

    let mut ore: OREAES128 = ORECipher::init(k1, k2, &seed).unwrap();
    let x_u64 = 100_u64.encrypt(&mut ore).unwrap();
    let y_u64 = 100983939290192_u64.encrypt(&mut ore).unwrap();

    let x_u32 = 100_u32.encrypt(&mut ore).unwrap();
    let y_u32 = 10098393_u32.encrypt(&mut ore).unwrap();

    c.bench_function("encrypt-8", |b| b.iter(|| do_encrypt_64(25u64, black_box(&mut ore))));
    c.bench_function("encrypt-left-8", |b| b.iter(|| do_encrypt_left_64(25u64, black_box(&mut ore))));
    c.bench_function("compare-8", |b| b.iter(|| do_compare(black_box(&x_u64), black_box(&y_u64))));

    c.bench_function("encrypt-4", |b| b.iter(|| do_encrypt_32(25u32, black_box(&mut ore))));
    c.bench_function("encrypt-left-4", |b| b.iter(|| do_encrypt_left_32(25u32, black_box(&mut ore))));
    c.bench_function("compare-4", |b| b.iter(|| do_compare(black_box(&x_u32), black_box(&y_u32))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);