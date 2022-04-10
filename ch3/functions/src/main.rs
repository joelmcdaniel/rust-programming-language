/* fn main() {
    //println!("Hello, world!");

    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
} */

/* fn main() {
    let x = 5;  // statements end with semicolons and do not return values

    let y = {   // everything between {} here is an expression
        let x = 3;
        x + 1       // expressions do not end with semicolons and return values
    };

    println!("The value of y is: {}", y);
} */

/* fn five() -> i32 {
    5   // functions return the last expression implicitly
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
} */

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    //x + 1; // mismatched type error: expected i32, found (),
    x + 1
}
