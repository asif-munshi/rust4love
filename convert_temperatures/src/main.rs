use std::io;

fn main() {
    println!("Convert Temperature!");

    'choosing_option: loop {
        println!();
        println!("1. Fahrenheit -> Celsius");
        println!("2. Celsius -> Fahrenheit");
        println!("Choose an option (1 or 2):");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // TODO handle other options
        // if option != 1 {
        //     continue;
        // }

        loop {
            println!();
            println!("Enter Temperature:");

            let mut temperature = String::new();

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

            let temperature: f64 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if option == 1 {
                //Fahrenheit °F to Celsius °C Formula
                let celsius = (temperature - 32.0) / 1.8;
                println!("{temperature}°F is {celsius}°C");
                break;
            } else {
                //Celsius °C to Fahrenheit °F Formula
                let fahrenheit = (temperature * 1.8) - 32.0;
                println!("{temperature}°C is {fahrenheit}°F");
                break;
            }
        }
        break 'choosing_option;
    }
}
