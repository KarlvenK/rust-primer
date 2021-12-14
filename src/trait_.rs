use crate::utils::put_blank_line;
use std::fmt::Debug;

pub fn practice() {
    let c = Circle {
        x: 0_f64,
        y: 0_f64,
        radius: 1_f64,
    };
    println!("circle c has an area of {}", c.area());

    fn foo<T: Debug>(s: T) {
        println!("{:?}", s);
    }
    foo("hehe");

    fn bar<T: Debug + Clone>(s: T) {
        s.clone();
        println!("{:?}", s);
    }
    bar("haha");

    fn foobar<T: Clone, K: Clone + Debug>(x: T, y: K) {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }

    fn foobar_<T, K>(x:T, y: K)
        where T: Clone, K: Clone + Debug {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }
    put_blank_line();
    
    internal_type();
    default_fn();
    inherit();
    derive_();
    trait_obj();
}

fn trait_obj() {
    trait Foo {
        fn method(&self) ->String;
    }
    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Foo for String {
        fn method(&self) -> String {
            format!("string: {}", *self)
        }
    }

    fn do_something(x: &dyn Foo) {
        x.method();
    }

    let x = "Hello".to_string();
    do_something(&x);
    let y = 8u8;
    do_something(&y);

    /* error
    let v = vec![1,2,3];
    let o = &v as &Clone;
    */


}

fn derive_() {
    //auto impl this traits:
    //Clone , Copy , Debug , Default , Eq , Hash ,
    // Ord , PartialEq , PartialOrd
    #[derive(Debug)]
    struct Foo;
    println!("{:?}", Foo);
    put_blank_line();
}

fn inherit() {
    trait Foo {
        fn foo(&self);
    }
    trait FooBar: Foo {
        fn foobar(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn foo(&self) {
            println!("foo");
        }
    }

    //FooBar 继承自 Foo 所以Baz必须实现了Foo才能实现FooBar

    impl FooBar for Baz {
        fn foobar(&self) {
            println!("foobar");
        }
    }
}

fn default_fn() {
    trait Foo {
        fn is_valid(&self) -> bool;
        //default fn for Foo's [ is_invalid() ]
        fn is_invalid(&self) -> bool {
            !self.is_valid()
        }
    }
}

fn internal_type() {
    trait HasAre {
        fn area(&self) -> f64;
    }
    impl HasAre for i32 {
        fn area(&self) -> f64 {
            *self as f64
        }
    }
    println!("{}", 5.area() * std::f64::consts::PI);
    put_blank_line();
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}