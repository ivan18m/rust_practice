// Vectors - resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers[2] = 20; // re-assign

    numbers.push(5);
    numbers.push(6);

    numbers.pop();

    println!("{:?}", numbers);
    println!("Vector value: {}", numbers[0]);
    println!("Vector length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Vector slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & Mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);

}