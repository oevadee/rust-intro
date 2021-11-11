// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, head-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello";
    let mut hello_allocated = String::from("Hello v2 ");

    println!("Lenght: {}", hello_allocated.len());

    hello_allocated.push('W');
    hello_allocated.push_str("orld!");

    println!("{}", hello);
    println!("{}", hello_allocated);
    println!("Capacity: {}", hello_allocated.capacity());
    println!("Is empty: {}", hello_allocated.is_empty());
    println!("Contains 'World:' {}", hello_allocated.contains("World"));
    println!("Replace: {}", hello_allocated.replace("World", "There"));

    for word in hello_allocated.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
}
