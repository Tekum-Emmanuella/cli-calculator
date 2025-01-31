pub struct Calculation {
    pub first: f64,
    pub operation: String,
    pub second: f64,
    pub result: Result<f64, String>,
}

pub fn calculate(first: f64, operation: &str, second: f64) -> Result<f64, String> {
    match operation {
        "+" => Ok(first + second),
        "-" => Ok(first - second),
        "*" => Ok(first * second),
        "/" => {
            if second == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(first / second)
            }
        },
        _ => Err("Invalid operation".to_string()),
    }
}
