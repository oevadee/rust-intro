use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("Array occupies {} bytes:", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];

    println!("Slice: {:?}", slice);
}
