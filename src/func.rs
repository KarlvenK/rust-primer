use std::ops::Index;
use crate::utils::put_blank_line;

pub fn practice() {
    say_hi("k");
    let xm = "xiaoming";
    let xh = "xiaohong";
    say_what(xm, hi);
    say_what(xh, hello);

    put_blank_line();
    pattern();
    /*
    fn main() {}
    fn main() -> () {}
    () is unit type
    () has no elements
     */
    hyper_func();

}

fn hyper_func() {
    fn inc(n: i32) -> i32 {
        n + 1
    }
    type IncType = fn(i32) -> i32;

    let func: IncType = inc;
    println!("3 + 1 = {}", func(3));
    println!("3 + 1 = {}", inc(3));

    put_blank_line();

    fn process(n: i32, func: IncType) -> i32 {
        func(n)
    }
    fn process_<F>(n: i32, func: F) -> i32
        where F: Fn(i32) -> i32 {
        func(n)
    }

    println!("9 + 1 = {}", process(9, inc));
    println!("9 + 1 = {}", process_(9, inc));

    put_blank_line();

    fn get_func(n: i32) -> fn(i32) -> i32 {
        fn inc(n: i32) -> i32 {
            n + 1
        }
        fn dec(n: i32) -> i32 {
            n - 1
        }

        if n % 2 == 0 {
            inc
        } else {
            dec
        }
    }

    let a = [1, 2, 3, 4, 5, 6, 7];
    let mut b = Vec::<i32>::new();
    for i in &a {
        b.push(get_func(*i)(*i))
    }
    println!("{:?}", b);
    put_blank_line();
}

fn pattern() {
    fn print_id((name, age): (&str, i32)) {
        println!("Im {}, age {}.", name, age);
    }
    fn print_age((_, age): (&str, i32)) {
        println!("my age is {}.", age);
    }
    fn print_name((name, _): (&str, i32)) {
        println!("Im {}", name);
    }


    let xm = ("xiaoming", 94);
    let xh = ("xiaohong", 84);
    print_name(xh);
    print_age(xh);
    print_id(xh);

    print_age(xm);
    print_name(xm);
    print_id(xm);

    struct Person {
        name: String,
        age: i32,
    }

    fn out_name(Person{name, ..}: Person) {
        println!("I am {}.",name);
    }
    let peter = Person {
        name: String::from("Peter"),
        age: 10,
    };
    out_name(peter);

    put_blank_line();
}

fn say_what(name: &str, func: fn(&str)) {
    func(name);
}

fn hi(name: &str) {
    println!("Hi, {}.", name);
}

fn hello(name: &str) {
    println!("Hello, {}.", name);
}

fn say_hi(name: &str) {
    println!("hi, {}", name);
}
