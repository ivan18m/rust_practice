pub fn run() {
    println!("Hello from print.rs file");
    println!("{} is the best {}", 1, "number");
    println!("Ivan is {0} and {1} but not {1} and {0}", "ayy", "lmao");
    println!("{name} is {age} and likes to play football", name="Ivan", age=5);
    println!("Binary {:b}, Hex: {:x}, Octal {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "hai"));
    println!("10 + 10 = {}", 10 + 10)
}