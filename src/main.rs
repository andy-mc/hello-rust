use std::io;

fn main() {
    println!("Enter the temperature:");

    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Failed to read line");

    let temp: f64 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        },
    };

    println!("Enter the units (C or F):");

    let mut units_input = String::new();
    io::stdin().read_line(&mut units_input).expect("Failed to read line");

    let units = units_input.trim();

    if units == "C" {
        let temp_f = temp * 9.0 / 5.0 + 32.0;
        println!("{}°C is equal to {}°F", temp, temp_f);
    } else if units == "F" {
        let temp_c = (temp - 32.0) * 5.0 / 9.0;
        println!("{}°F is equal to {}°C", temp, temp_c);
    } else {
        println!("Please enter a valid unit (C or F)");
    }
}
