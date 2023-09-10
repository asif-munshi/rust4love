fn main() {
    //if Expressions
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Not Zero
    if number != 0 {
        println!("number was something other than zero");
    }

    //Divisible by 4, 3 or 2
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Using if in a let Statement
    let condition = true;
    let int_value = if condition { 5 } else { 6 };

    println!("The value of number is: {int_value}");

    //Repeating Code with loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!();

    //Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!();

    println!("while:");
    //Conditional Loops with while
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", arr[index]);

        index += 1;
    }
    println!();

    println!("for:");
    //Conditional Loops with for
    for element in arr {
        println!("the value is: {element}");
    }
    println!();

    //Conditional Loops with for to reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
