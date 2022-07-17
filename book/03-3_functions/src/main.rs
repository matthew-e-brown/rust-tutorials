fn main() {
    // Expressions and statements are weird... I've never had to consider which
    // is which in any other language before...

    let y = {
        let x = 3;
        x + 1
    };

    println!("y is {}", y);

    let y = plus_one(y);

    println!("y is {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}