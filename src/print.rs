pub fn run() {
    println!("Text file");

    // Basic Formating
    println!("Number: {}", 1);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0}, likes to {2}",
        "Adi", "Szczechura", "code"
    );

    // Named args
    println!(
        "{name} likes to play {activity}",
        name = "Adi",
        activity = "Baseball"
    );

    // Touple
    println!("{:?}", (12, true, "hello"));
}
