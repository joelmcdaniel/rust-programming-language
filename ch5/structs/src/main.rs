#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit-like structs without any fields
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // user1.email = String::from("anotheremail@example.com");

    println!("user1 = {:#?}", user1);

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // must come last
    };

    println!("user2 = {:#?}", user2);

    // instantiate tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}; origin: {:?}", black, origin);

    let subject = AlwaysEqual;

    println!("subject: {:?}", AlwaysEqual);
}
