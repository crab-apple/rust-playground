use std::fmt::Error;

fn main() {
    // A method can return a result. The result can be successful
    let dividend = 15;
    let divisor = 5;
    let result = divide_exactly(dividend, divisor);

    println!(
        "{} divided by {} is {}",
        dividend,
        divisor,
        result.expect("Result should not be None")
    );

    // Or the result can fail
    let dividend = 15;
    let divisor = 4;
    let result = divide_exactly(dividend, divisor);

    println!(
        "{} divided by {} failed with {}",
        dividend,
        divisor,
        result.err().unwrap()
    );

    // You can do error handling with match
    println!("Dividing 9 by 3");
    match divide_exactly(9, 3) {
        Ok(v) => println!("The result is {}", v),
        Err(e) => println!("Error: {}", e),
    }
    println!("Dividing 9 by 2");
    match divide_exactly(9, 2) {
        Ok(v) => println!("The result is {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide_exactly(dividend: i64, divisor: i64) -> Result<i64, String> {
    if dividend % divisor == 0 {
        return Ok(dividend / divisor);
    }
    Err("Not divisible".to_string())
}
