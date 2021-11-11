pub fn run() {
    let age: u8 = 17;
    let check_id: bool = true;

    if age >= 18 && check_id {
        println!("You can drink");
    } else if age < 18 && check_id {
        println!("You can't drink");
    } else {
        println!("Need ur ID");
    }

    // Shorter
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
