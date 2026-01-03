use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(v) = self.values.get(&arg) {
            *v
        } else {
            let v = (self.calculation)(arg);
            self.values.insert(arg, v);
            v
        }
    }
}

fn main() {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    println!("Value: {}", expensive_result.value(40));
    println!("Value: {}", expensive_result.value(50));
    println!("Value: {}", expensive_result.value(40)); // cached
}
