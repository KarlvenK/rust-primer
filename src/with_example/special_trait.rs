use std::array::IntoIter;
use std::mem::take;
use crate::utils::put_blank_line;

pub fn practice() {
    derive();
    dyndyn();
    ite();
    impl_trait();
    impl_clone();
    super_trait();
    eliminate_overlapped_trait();
}

fn eliminate_overlapped_trait() {
    trait UsernameWidget {
        fn get(&self) -> String;
    }

    trait AgeWidget {
        fn get(&self) -> u8;
    }

    struct Form {
        username: String,
        age: u8,
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }
    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: String::from("a"),
        age: 1,
    };

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!(String::from("a"), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(1, age);
}

fn super_trait() {
    trait Person {
        fn name(&self) -> String;
    }

    trait Student: Person {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorate language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username(),
        )
    }
}

fn impl_clone() {
    put_blank_line();

    #[derive(Debug, Clone, Copy)]
    struct Nil;
    #[derive(Debug, Clone)]
    struct Pair(Box<i32>, Box<i32>);

    let nil = Nil;
    let copied_nil = nil;
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);
    //--------
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    //error Pair(1,2) has been moved to moved_pair
    //println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);
    //error it has been dropped
    //println!("copy: {:?}", moved_pair);
    println!("clone: {:?}", cloned_pair);
}

fn impl_trait() {
    put_blank_line();

    use std::iter;
    use std::vec::IntoIter;

    fn combine_vecs_explict_return_type (
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>,IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    fn combine_vecs (
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> impl Iterator<Item=i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());

    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move|x: i32| {
            x + y
        };
        closure
    }

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    fn double_positive<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers
            .iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    }

    println!("all done");
}

fn ite() {
    struct Fib {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fib {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }

    fn fib() -> Fib {
        Fib {
            curr: 1,
            next: 1,
        }
    }

    let mut sequence = 0..3;
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    for i in fib().take(4) {
        println!("> {}", i);
    }
    put_blank_line();
    for i in fib().skip(7).take(9) {
        println!("> {}", i);
    }

}


fn dyndyn() {
    put_blank_line();

    struct Sheep{};
    struct Cow{};
    trait Animal {
        fn noise(&self) -> &'static str;
    }

    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaaaaaaah!"
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "mooooooooo!"
        }
    }

    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep{})
        } else {
            Box::new(Cow{})
        }
    }

    let random = 0.2345;
    let animal = random_animal(random);
    println!("you get an animal and it says {}", animal.noise());
}


fn derive() {
    put_blank_line();

    #[derive(PartialEq, PartialOrd)] //comparable struct
    struct Centimeters(f64);
    #[derive(Debug)] //printable struct
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;
            Centimeters(inches as f64 * 2.54)
        }
    }
    struct Second(i32);

    let _one_second = Second(1);
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter", cmp);

}