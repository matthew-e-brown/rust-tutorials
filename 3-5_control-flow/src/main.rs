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

    println!("result is {}!", result);
}