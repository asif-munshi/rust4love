fn main() {
    another_function(5);
    print_labeled_measurement(5, 'm');
    let value = return_value();
    let value_plus_one = return_plus_one(5);

    println!("The returned value is: {value}");
    println!();
    println!("The returned plus one value is: {value_plus_one}");

    let func = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {func}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//Functions with Return Values
fn return_value() -> i32 {
    5
}

fn return_plus_one(x: i32) -> i32 {
    x + 1
}
