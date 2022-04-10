// String
/* fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
} */

// s1 is moved into s2
/* fn main() {
    let s1 = String::from("hello");
    let s2 = s1;    // s1 is now invalid, moved into s2

    println!("{}, world!", s1) // will produce error[E0382]: borrow of moved value: `s1`
} */

// Clone (deep copy - heap)
/* fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();    // String heap data gets copied

    println!("s1 = {}, s2 = {}", s1, s2)
} */

// Shallow copy - stack; i32 is Copy
/* fn main() {
    let x = 5;
    let y = x; // x still valid, not moved into y

    println!("x = {}, y = {}", x, y);
} */

// Ownership and Functions
/* fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    //println!("s (String) invalid/moved. s = {}", s); // error[E0382]: borrow of moved value: `s`

    let x = 5;          // x comes into scope

    makes_copy(x);      // x would move into the function,
                        // but i32 is Copy, so it’s okay to still
                        // use x afterward

    println!("x (i32) is Copy so ok here. x = {}", x); // ok, just fine
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens. */

// Return Values and Scope
/* fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.


fn gives_ownership() -> String {            // gives_ownership will move its
                                            // return value into the function
                                            // that calls it

    let some_string = String::from("hello");    // some_string comes into scope

    some_string                             // some_string is returned and
                                            // moves out to the calling
                                            // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
} */

/* // return multiple values using a tuple
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
} */

/* // References and Borrowing
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens. */

// References also immutable by default
/* fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");    // error[E0596]: cannot borrow `*some_string` as mutable
} */

// Mutable References
/* fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
} */

// one mutable reference to a particular piece of data in a
// particular scope. error[E0499]: cannot borrow `s` as mutable
// more than once at a time
/* fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
} */

// use curly brackets to create a new scope, allowing for
// multiple mutable references, just not simultaneous ones
/* fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("{}", r2);
} */

// error[E0502]: cannot borrow `s` as mutable because it is also
// borrowed as immutable
/* fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
} */

// a reference's scope starts from where it is introduced and
// continues through the last time that reference is used. This
// code will compile because the last usage of the immutable
// references occurs before the mutable reference is introduced
/* fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
} */

// Compiler won't allow dangling references
/* fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger! */

// Solution to above is to return the String directly
fn main() {
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
