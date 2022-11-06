pub fn run() {
    let name = "Brad Pitt";
    let mut age = 27;
    age = 28;
    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Brad Pitt", 27);
    println!("{} is {}", my_name, my_age);
}