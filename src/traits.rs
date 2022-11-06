trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Fly you fools!")
    }
}

impl Human {
    fn fly(&self) {
        println!("Human Waves hands");
    }
}

pub fn run() {
    let person: Human = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}