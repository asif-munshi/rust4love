use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5];

    loop {
        println!("Please enter an array index:");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let element = arr[index];

        println!();
        println!("The value of the element at index {index} is: {element}");
        break;
    }
}
