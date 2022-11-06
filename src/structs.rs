// Structs - Used to create custom data types

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 128
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c_t = ColorTuple(255, 0, 128);
    c_t.0 = 200;
    println!("Color Tuple: {} {} {}", c_t.0, c_t.1, c_t.2);

    let mut p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);

    let mut v = Person::new("Victoria", "Doe");
    v.set_last_name("Williams");
    println!("Person: {}", v.full_name());
    println!("Peron Tuple {:?}", v.to_tuple());
}