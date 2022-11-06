// Primitive str - Immutable, fixed length
// String = Growable, heap allocated structure

pub fn run() {
    let mut hello = String::from("Hello");
    hello.push(' '); // append char
    hello.push_str("world!"); // append str
    println!("{}", hello);
    println!("Length: {}", hello.len());
    println!("Capacity: {}", hello.capacity());
    println!("Contains 'World' {}", hello.contains("world"));
    println!("Replace: {}", hello.replace("world", "universe"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut str = String::with_capacity(10);
    str.push('a');
    assert_eq!(1, str.len());
    assert_eq!(10, str.capacity());
    println!("{}", str);
}