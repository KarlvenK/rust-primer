use std::thread;
use std::time::Duration;
use crate::utils::put_blank_line;

pub fn practice() {
    capture();
    as_input_parameter()
}

fn as_input_parameter() {
    put_blank_line();

    fn apply<F>(f: F) where F: FnOnce() {
        f();
    }
    
}

fn capture() {
    put_blank_line();

    use std::mem;
    let color = "green";
    let print = ||println!("`color`: {}", color);
    print();
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    //consume();

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&2));

    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(5, num);

    struct Cacher<T>
        where T: Fn(u32) -> u32 {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32 {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num + 1
        });

        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                expensive_result.value(intensity)
            );
            println!(
                "Next, do {} situps!",
                expensive_result.value(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    generate_workout(70, 10);
}