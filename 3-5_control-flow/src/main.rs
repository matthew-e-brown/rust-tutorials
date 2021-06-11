use rand::Rng;

fn main() {
    let mut counter = 0;
    let stopper = rand::thread_rng().gen_range(10..=50);

    println!("Looping...");

    let result = loop {
        counter += 1;

        if counter == stopper {
            break counter * 2;
        }
    };

    let x = if result > 10 { 100 } else { 200 };

    println!("result is {}!", result);
    println!("x is {}!", x);

    println!("five() == {}", five());

    let y = {
        counter = 0;
        let result = loop {
            counter += 1;

            if counter == stopper {
                break counter * 2;
            }
        };

        if result > 10 { 100 } else { 300 }
    };

    println!("y is {}!", y);
}


fn five() -> u32 {
    5
}