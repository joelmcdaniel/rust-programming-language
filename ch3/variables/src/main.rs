#![allow(unused)]

use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/* fn main() {
    // Variables immutiable by default,
    // use mut keyword for mutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is {}", x);

    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS)
} */

/* fn main() {
    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    // error not allowed to mutate variables types
    //let mut spaces = "   ";
    //spaces = spaces.len();

    println!("Number of spaces: {}", spaces)
} */

/* fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
} */

/* fn main() {
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
} */

/* fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
} */

/* fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
} */

/* fn main() {
    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values of x, y and z are: {}, {}, {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "The values of x, y and z are: {}, {}, {}",
        five_hundred, six_point_four, one
    )
} */

/* fn main() {
    // Arrays
    let a = [1, 2, 3, 4, 5];

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

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    /* let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element); // panic 'index out of bounds' */

    let index = 4;

    let element = a[index];

    println!("The value of element is: {}", element);
} */

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
