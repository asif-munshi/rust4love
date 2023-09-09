fn main() {
    //mutability
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");
    println!();

    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 Hours in Seconds: {THREE_HOURS_IN_SECONDS}");
    println!();

    //Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of space is: {spaces}");
}
