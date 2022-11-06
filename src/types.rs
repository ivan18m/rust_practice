/**
 * Integers: u8, i8 - u128, i128
 * Floats: f32, f64
 * Arrays (static)
 * Vector (dynamic)
 * char, string, bool, tuple...
 */

pub fn run() {
    let x = 1; // default is i32
    let y = 2.5; // default is f64

    let z: i64 = 7337373731801;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;
    let is_greater = x > 1;

    let c = 'x';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, x == 1, c, face))
}