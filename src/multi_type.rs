pub mod tuples {
    pub fn practice() {
        let y = (2, "hello");
        let x: (i32, &str) = (3, "world");

        let (w, z) = y;
        println!("{} {}", w, z);
        println!("{:?}", x);

        let f = x.0;
        let e = x.1;
        println!("{} {}", f, e);
    }
}

pub mod structure {
    /*
    struct A {
        attr2: String,
    }
    struct B(i32, u16, bool);
    struct D;
    */
    struct Person {
        name: String,
    }

    impl Person {
        fn new(n: &str) -> Person {
            Person {
                name: n.to_string(),
            }
        }

        fn greeting(&self) {
            println!("{} say hello.", self.name);
        }
    }

    #[derive(Copy, Clone)]
    struct A {
        a: i32,
    }

    impl A {
        pub fn show(&self) {
            println!("{}", self.a);
        }
        pub fn add_two(&mut self) {
            self.add_one();
            self.add_one();
            self.show();
        }
        pub fn add_one(&mut self) {
            self.a += 1;
        }
    }

    pub fn practice () {
        let peter = Person::new("Peter");
        peter.greeting();

        let mut ast = A {
            a: 12,
        };
        ast.show();
        ast.add_two();
    }
}

pub mod enumerate {

    enum SpecialPoint <'a> {
        Point(i32, i32),
        Special(&'a str),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    use crate::utils::put_blank_line;
    pub fn practice() {
        put_blank_line();
        let sp = SpecialPoint::Point(0, 0);
        match sp {
            SpecialPoint::Point(x,y) => {
                println!("I am SpecialPoint(x={}, y={})", x, y);
            }
            SpecialPoint::Special(why) => {
                println!("I am Special because I am {}", why);
            }
        }
        let sp = SpecialPoint::Special("the origin point");
        match sp {
            SpecialPoint::Point(x,y) => {
                println!("I am SpecialPoint(x={}, y={})", x, y);
            }
            SpecialPoint::Special(why) => {
                println!("I am Special because I am {}", why);
            }
        }

        put_blank_line();

        let point = Point {
            x: 1,
            y: 2,
        };

        let Point{x: xx, y: yy} = point;
        // or
        let Point{x, y} = point;

        println!("{} {}", xx, yy);

    }
}