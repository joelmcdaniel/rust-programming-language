#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function that is not a method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

    println!("The area of rectangle 1 is {} sqare pixels.", rect1.area());

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        ..rect1
    };

    let rect3 = Rectangle {
        width: 65,
        height: 55,
    };

    dbg!(&rect2);

    println!("The area of rectangle 2 is {} sqare pixels.", rect2.area());

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));
    println!("Can rect3 hold rect1? {}", rect3.can_hold(&rect1));

    println!(
        "Sqaures are rectangles too! square: {:?}",
        Rectangle::square(3)
    )
}
