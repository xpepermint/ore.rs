use small_prp::prp::Prp;
use hex_literal::hex;

#[test]
fn init_prp() {
    let key: [u8; 16] = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    let prp = Prp::init(&key);

    // TODO: Test all numbers in the block
    println!("15 -> {}", prp.permute(15));
    println!("75 -> {}", prp.permute(75));
    assert_eq!(15, prp.inverse(prp.permute(15)));
}

// TODO: Add tests for the different valid block sizes