fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // shadowing is happening here
    let x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;


    // data types

    // integer types
    // decimal
    let x = 98_222;
    // hexadecimal
    let x = 0xff;
    // octal
    let x = 0o77;
    // binary
    let x = 0b1111_0000;
    // byte (u8 only)
    let x = b'A';

    let f: u8 = 255; // 256 will becomes 1 because of unsigned 8 bytes

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple
    let tup = (500, 6.4, 1);
    let (channel, sub_count, sub_count1) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    // array initliazation with value before ; and size after ;
    let byte = [0;8];


    // functions
    let sum = my_function(2,3);

    // Control flow
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            // this will break the loop and return counter
            break counter;
        }
    };

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // Line comment

    /*
    Block comment
    */
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let sum = x + y;
    return sum;

    // alternative return syntax
    // x + y
}
