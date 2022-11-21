use std::{rc::Rc, sync::Arc};

use crate::print;

trait Vehicle {
    fn drive(&self);
}

struct Car;

impl Vehicle for Car {
    fn drive(&self) {
        println!("Car is driving")
    }
}

#[derive(Debug)]
struct Truck {
    capacity: i32,
}

pub fn run() {
    // Box - allocate mem to Heap instead of stack
    // stack mem is much faster
    // on stack compiler needs to know exact amount of mem to allocate
    // in this case it's not known as anything can implement trait Vehicle
    let c: Box<dyn Vehicle>;
    // When referencing a trait we need to use dyn
    c = Box::new(Car);
    c.drive();

    // Next example
    // Rc - reference counter
    // we want mem to be deallocated when ref count reaches 0
    let (truck_a, truck_b, truck_c) = (
        Rc::new(Truck { capacity: 1 }),
        Rc::new(Truck { capacity: 2 }),
        Rc::new(Truck { capacity: 3 }),
    );

    let facility_one = vec![Rc::clone(&truck_a), Rc::clone(&truck_b)];
    let facility_two = vec![Rc::clone(&truck_b), Rc::clone(&truck_c)];

    println!("Facility one: {:?}", facility_one);
    println!("Facility two: {:?}", facility_two);
    println!("Truck b strong count: {:?}", Rc::strong_count(&truck_b));

    std::mem::drop(facility_two);
    println!(
        "Truck b strong count after drop: {:?}",
        Rc::strong_count(&truck_b)
    );

    // Next example
    // Arc - async reference counter
    // Uses atomic data (thread safe) types that compiler knows how to order
    // not as efficient as Rc
    let (truck_d, truck_e, truck_f) = (
        Arc::new(Truck { capacity: 1 }),
        Arc::new(Truck { capacity: 2 }),
        Arc::new(Truck { capacity: 3 }),
    );

    let thread = std::thread::spawn(move || {
        let facility_three = vec![Arc::clone(&truck_d), Arc::clone(&truck_e)];
        let facility_four = vec![Arc::clone(&truck_e), Arc::clone(&truck_f)];
        (facility_three, facility_four)
    });

    let (facility_three, facility_four) = thread.join().unwrap();

    println!("Facility three: {:?}", facility_three);
    println!("Facility four: {:?}", facility_four);

    // truck_e ownership at thread, clone 2nd elem of facility three
    let truck_e = Arc::clone(&facility_three[1]);
    println!("Truck e strong count: {:?}", Arc::strong_count(&truck_e));

    std::mem::drop(facility_four);
    println!(
        "Truck e strong count after drop: {:?}",
        Arc::strong_count(&truck_e)
    );
}
