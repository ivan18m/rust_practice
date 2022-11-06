// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // Non-Primitives will lose values
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // vec1 is borrowed to vec2
    let vec1 = &vec2; // vec1 is returned from vec2

    println!("Values: {:?}", (vec1, &vec2));
}
