use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Add on to vector
    numbers.push(6);
    numbers.push(6);

    println!("{:?}", numbers);
    println!("Vector occupies {} bytes:", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];

    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
