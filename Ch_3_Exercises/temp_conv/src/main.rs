use std::io;

fn main() {
    let mut units = String::new();
    let fahrenheit: String = String::from("f");
    let celcius: String = String::from("c");

    loop {
        println!("Enter temperature units (F or C): ");
        io::stdin().read_line(&mut units).expect("Failed to read input");
        let mut _units: String = units.trim().parse().expect("Please type a letter!");
        println!("DEBUG/// Units {} == F: {} / == C: {}", _units.to_lowercase(), _units.to_lowercase().eq(&fahrenheit), _units.to_lowercase().eq(&celcius));
        println!("DEBUG/// Break statement is: {}", _units.to_lowercase().eq(&fahrenheit) || _units.to_lowercase().eq(&celcius));
        if _units.to_lowercase().eq(&fahrenheit) || _units.to_lowercase().eq(&celcius) {
            break;
        }
        else {
            println!("Please try again...");
            units.clear();
            _units.clear();
        }
    }

    println!("Enter temperature value: ");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read input");
    let value: u32 = value.trim().parse().expect("Please type a number!");

    if units.eq(&fahrenheit){
        //Convert Fahrenheit to Celcius
        let req_value: f32 = ((value - 32)*(5/9)) as f32;
        println!("Converting {} degrees Fahrenheit to {} degrees Celsius", value, req_value);
    } else {
        //Convert Celcius to Farenheit
        let req_value: f32 = (((9/5)*value) + 32) as f32;
        println!("Converting {} degrees Celsius to {} degrees Fahrenheit", value, req_value);
    }
}
