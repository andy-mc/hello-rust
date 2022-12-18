use std::io;

fn main() {
    let temp = input_temp();
    let units = input_units();

    match (temp, units) {
        (Ok(temp), Ok(units)) => {
            if units == "C" {
                let temp_f = temp * 9.0 / 5.0 + 32.0;
                println!("{:.2}°C is equal to {:.2}°F", temp, temp_f);
            } else if units == "F" {
                let temp_c = (temp - 32.0) * 5.0 / 9.0;
                println!("{:.2}°F is equal to {:.2}°C", temp, temp_c);
            } else {
                println!("Please enter a valid unit (C or F)");
            }
        },
        (Err(e), _) => println!("Error reading temperature: {}", e),
        (_, Err(e)) => println!("Error reading units: {}", e),
    }
}

fn input_temp() -> Result<f64, String> {
    println!("Enter the temperature:");

    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).map_err(|e| e.to_string())?;

    temp_input.trim().parse().map_err(|e| e.to_string())
}

fn input_units() -> Result<String, String> {
    println!("Enter the units (
