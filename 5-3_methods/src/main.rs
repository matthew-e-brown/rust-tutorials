#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 5, height: 10 };
    let rect2 = Rectangle { width: 10, height: 15 };

    println!("The area of a {:?} is {}", rect1, rect1.area());
    println!("The area of a {:?} is {}", rect2, rect2.area());

    println!(
        "Can rect2 fit inside rect1? {}",
        if rect1.can_contain(&rect2) { "Yes." } else { "No." }
    );
    println!(
        "Can rect1 fit inside rect2? {}",
        if rect2.can_contain(&rect2) { "Yes." } else { "No." }
    );

    let square1 = Rectangle::square(15);
    let square2 = Rectangle::square(25);

    println!(
        "Can square2 fit inside square1? {}",
        if square1.can_contain(&square2) { "Yes." } else { "No." }
    );
    println!(
        "Can square1 fit inside square2? {}",
        if square2.can_contain(&square1) { "Yes." } else { "No." }
    );
}