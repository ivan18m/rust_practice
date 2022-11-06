pub fn run() {
    greeting("Henlo", "Stranger");
    // Bind function vals to vars
    let get_sum = add(4, 6);
    println!("Sum: {}", get_sum);

    // Closure -  something like lambdas
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // return has no ;
    n1 + n2
}