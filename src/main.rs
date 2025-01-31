mod calculator;

use std::io;
use calculator::{calculate, Calculation};

fn main() {
    println!("Simple CLI Calculator");
    let mut history: Vec<Calculation> = Vec::new();
    
    loop {
        let first_number = get_user_input("Enter first number: ");
        let operation = get_user_input("Enter operation (+, -, *, /): ");
        let second_number = get_user_input("Enter second number: ");
        
        let first: f64 = first_number.parse().unwrap_or(0.0);
        let second: f64 = second_number.parse().unwrap_or(0.0);
        
        let result = calculate(first, &operation, second);
        
        let calc = Calculation {
            first,
            operation: operation.clone(),
            second,
            result: result.clone(),
        };
        history.push(calc);
        
        match result {
            Ok(r) => println!("Result: {}", r),
            Err(e) => println!("Error: {}", e),
        }
        
        if !continue_calculation() {
            break;
        }
    }
    
    // Print calculation history
    println!("\nCalculation History:");
    for (i, calc) in history.iter().enumerate() {
        println!("{}. {} {} {} = {:?}", i+1, calc.first, calc.operation, calc.second, calc.result);
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn continue_calculation() -> bool {
    let mut input = String::new();
    println!("Do you want to perform another calculation? (y/n): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_lowercase() == "y"
}
