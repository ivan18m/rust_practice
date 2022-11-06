pub fn run() {
    let mut count = 0;
    // infinite loop
    // loop {
    //     count+=1;
    //     println!("Number: {}", count);

    //     if count == 2 {
    //         break;
    //     }
    // }

    // while count <= 6 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     }
    //     else if count % 3 == 0 {
    //         println!("fizz");
    //     }
    //     else if count % 5 == 0 {
    //         println!("buzz");
    //     }
    //     else {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    for x in 0..6 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        }
        else if x % 3 == 0 {
            println!("fizz");
        }
        else if x % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", x);
        }
    }
}