/* fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
} */

fn main() {
    let condition = true;
    // if is an expression so can be used on the right side
    // of a let statement
    let number = if condition {
        5
    } else {
        6
        // "six" // error: if and else have incompatible types
    };

    println!("The value of number is: {}", number);
}
