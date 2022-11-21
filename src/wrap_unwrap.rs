use std::num::ParseIntError;
#[derive(Debug)]
struct SummationError;

fn to_int(s: &str) -> Result<i32, ParseIntError> {
    s.parse()

    // When we unwrap we either get Ok value or panic in case of Error option
    // s.parse().unwrap // return i32

    // Same as unwrap but you can specify custom err message
    // s.parse().expect("issue converting str to int!") // return i32

    // in case of err, use 0
    //s.parse().unwrap_or(0) // return i32

    // return an Ok Option, it can me Some or None
    //s.parse().ok() // return Option<i32>
}

fn sum_str_vec(strs: Vec<String>) -> Result<String, SummationError> {
    let mut accum = 0i32;
    for s in strs {
        // ? question mark => if Result Option is Ok then add to accum, if Err propagate to stack
        // map_err converts ParseIntError to SumationError
        accum += to_int(&s).map_err(|_| SummationError)?;
    }

    Ok(accum.to_string())
}

pub fn run() {
    let v = vec![String::from("3"), String::from("4")];
    let total = sum_str_vec(v);
    println!("{:?}", total);

    // abc should not parse to int
    let v = vec![String::from("3"), String::from("abc")];
    let total = sum_str_vec(v);
    println!("{:?}", total);
}
