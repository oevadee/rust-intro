// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Touple Struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to touple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.green = 200;

    let mut c2 = Color2(255, 0, 0);
    c2.2 = 120;

    let p = Person::new("Adi", "Szczechura");

    let mut p2 = Person::new("Jacek", "Szwarc");

    println!("Color: {} {} {}", c.red, c.green, c.blue);
    println!("Color2: {} {} {}", c2.0, c2.1, c2.2);
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person full name method {}", p2.full_name());
    p2.set_last_name("Kwarc");
    println!("Person full name method {}", p2.full_name());
    println!("Person touple {:?}", p2.to_tuple());
}
