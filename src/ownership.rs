use crate::utils::put_blank_line;

pub fn practice() {
    owner();
    copytrait();
    advanced_copy_trait();
    ref_and_borrow();
    life_time();
}

fn life_time() {
    struct Person<'a> {
        age: &'a u8,
    }

    impl<'a> Person<'a> {
        fn print_age(&self) {
            println!("Person.age = {}", self.age);
        }

        fn get_age(&self) -> &u8 {
            self.age
        }
    }
    let x = 20_u8;
    let t = Person {
        age: &x,
    };

}

fn barr<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    let n = 1;
    if n > 6 {
        x
    } else {
        y
    }
}

fn bar<'a>(x: &'a str, y: &'a str) -> &'a str {
    let n = 1;
    if n > 6 {
        x
    } else {
        y
    }
}

fn foo(x: &str) -> &str {
    x
}
/*
implicitly:

fn foo<'a>(x: &'a str) -> &'a str {
    x
}

 */

fn ref_and_borrow() {
    let x:Vec<i32> = vec!(1,2,3,4);
    let y = &x;
    println!("x={:?}, y={:?}", x, y);

    let mut x:i32 = 100;
    {
        let y: &mut i32 = &mut x;
        *y += 2;
    }
    println!("{}", x);
    put_blank_line();
}

fn advanced_copy_trait() {
    /*
    struct Foo { // 可实现Copy trait,因为i32和bool都实现了Copy trait
        a: i32,
        b: bool,
    }
    struct Bar { //不可实现Copy trait，因为Vec<i32>没有实现Copy trait
        l: Vec<i32>,
    }
    */
    #[derive(Copy, Clone)]
    struct Foo {
        a: i32,
        b: bool,
    }
    // or
    #[derive(Debug)]
    struct Fooo {
        a: i32,
        b: bool,
    }
    impl Copy for Fooo {}
    impl Clone for Fooo {
        fn clone(&self) -> Fooo {
            Fooo{a: self.a, b: self.b}
        }
    }

    let x = Fooo{a:100, b:true};
    let mut y = x;
    y.b = false;
    println!("{:?}", x);
    println!("{:?}", y);
    put_blank_line();
}

fn copytrait() {
    let a: i32 = 666666;
    //a <=> ram[ptr:A, content:666666]
    let b = a;
    //a <=> ram[ptr:A, content:666666]
    //b <=> ram[ptr:B, content:666666]
    println!("{}",a);

    let a: String = String::from("xyz");
    //a <=> ram[ptr:A, content:"xyz"]
    let b = a.clone();
    //a <=> ram[ptr:A, content:"xyz"]
    //b <=? ram[ptr:B, content:"xyz"]
    println!("{}", a);

    put_blank_line();
}

fn owner() {
    put_blank_line();
    let a: i32;
    //a <=> ram[ptr:A, content: empty]
    a = 10086;
    //a <=> ram[ptr:A, content:10086]
    println!("{}", a);


    let a: String = String::from("xyz");
    //a <=> ram[ptr:A, content:"xyz"]
    let b = a;
    //a
    //b <=> ram[ptr:A, content:"xyz"]
    println!("{}", b);
    put_blank_line();
}