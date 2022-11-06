use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    numbers[2] = 20; // re-assign

    println!("{:?}", numbers);
    println!("Array value: {}", numbers[0]);
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Array slice: {:?}", slice);
}