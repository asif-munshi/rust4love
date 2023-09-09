fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);
    println!();

    //Scalar Types
    //Integer Types
    //Signed Integer
    let int_signed_8bit: i8 = 127;
    let int_signed_16bit: i16 = 32767;
    let int_signed_32bit: i32 = 2147483647;
    let int_signed_64bit: i64 = 9223372036854775807;
    let int_signed_128bit: i128 = 170141183460469231731687303715884105727;

    println!("8-bit Signed Integer Range: -128..={int_signed_8bit}");
    println!("16-bit Signed Integer Range: -32768..={int_signed_16bit}");
    println!("32-bit Signed Integer Range: -2147483648..={int_signed_32bit}");
    println!("64-bit Signed Integer Range: -9223372036854775808..={int_signed_64bit}");
    println!("128-bit Signed Integer Range: -170141183460469231731687303715884105728..={int_signed_128bit}");
    println!();

    //Unsigned Integer
    let int_unsigned_8bit: u8 = 255;
    let int_unsigned_16bit: u16 = 65535;
    let int_unsigned_32bit: u32 = 4294967295;
    let int_unsigned_64bit: u64 = 18446744073709551615;
    let int_unsigned_128bit: u128 = 340282366920938463463374607431768211455;

    println!("8-bit Unsigned Integer Range: 0..={int_unsigned_8bit}");
    println!("16-bit Unsigned Integer Range: 0..={int_unsigned_16bit}");
    println!("32-bit Unsigned Integer Range: 0..={int_unsigned_32bit}");
    println!("64-bit Unsigned Integer Range: 0..={int_unsigned_64bit}");
    println!("128-bit Unsigned Integer Range: 0..={int_unsigned_128bit}");
    println!();

    //Integer Literals
    let decimal_literal = 98_222;
    let hex_literal = 0xff;
    let octal_literal = 0o77;
    let binary_literal = 0b1111_0000;

    println!("Decimal Literal: {decimal_literal}");
    println!("Hex Literal: {hex_literal}");
    println!("Octal Literal: {octal_literal}");
    println!("Binary Literal: {binary_literal}");
    println!();

    //Floating-Point Types
    let f64_float = 2.0; // f64
    let f32_float: f32 = 3.0; // f32

    println!("f64: {f64_float} and f32: {f32_float} Floating-point numbers.");
    println!();

    //Boolean Type
    let bool_true = true;
    let bool_false: bool = false; // with explicit type annotation

    println!("Boolean Type: {bool_true}");
    println!("Boolean Type: {bool_false}");
    println!();

    //Character Type
    let character = 'z';
    let annotated_char: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Character: '{character}', explicit type annotation: '{annotated_char}', Emoji: {heart_eyed_cat}");
    println!();

    //Compound Types
    //Tuple Type
    let tup = (500, 6.4, 1);

    let (tup_one, tup_two, tup_three) = tup;

    println!("The value of First Tuple Type is: {tup_one}");
    println!("The value of Second Tuple Type is: {tup_two}");
    println!("The value of Third Tuple Type is: {tup_three}");
    println!();

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The value of Tuples: {five_hundred}, {six_point_four}, {one}");
    println!();

    //Array Type
    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Array One: {:?}", arr);
    println!("Months: {:?}", months);
    println!();

    //Accessing Array Elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let fifth = a[4];

    println!("First element of array: {first}");
    println!("Fifth element of array: {fifth}");
}
