use std::io;

fn main () {
    println!("Simple Calculator");
    println!("====================");

    // get the first number
    println!("Enter the first number");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read input");
    let first_number: f64 = match first_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return;
        }
    };

    // get the second number
    println!("Enter the second number");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read input");
    let second_number: f64 = match second_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    // get the operation
    println!("Enter operator");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

    // get the result
    let result: f64 = match operation {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => {
            if second_number == 0.0 {
                println!("Division by zero is not allowed");
                return;
            }
            first_number / second_number
        }
        _ => {
            println!("Please enter the correct operator from these +, -, *, or /.");
            return;
        }
    };

    println!("The result is {}", result)
}