use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct CacheRunner<T> where T: Fn(u32) -> u32 {
    calculator: T,
    map: HashMap<u32, u32>,
}

impl<T> CacheRunner<T> where T: Fn(u32) -> u32 {
    fn new(calculator: T) -> CacheRunner<T> {
        CacheRunner {
            calculator,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.map.get(&arg) {
            Some(val) => *val,
            None => {
                let new_val = (self.calculator)(arg);
                self.map.insert(arg, new_val);
                new_val
            }
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = CacheRunner::new(|num| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}