/*
    Primitive Types:
        Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
        Floats: f32, f64
        Boolean: (bool)
        Characters: (char)
        Tuples
        Arrays
*/

pub fn run() {
    let x = 1;
    let y = 2.5;
    let z: i64 = 712347812378912;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active: bool = true;

    let a1 = 'a';
    let face = '\u{1F631}';

    println!("{:?}", (is_active, a1, face));
}
