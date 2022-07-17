#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 15,
        height: 25
    };

    println!(
        "The area of a {:?} rectangle is {}",
        rect, area(&rect)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}