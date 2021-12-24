use std::f32::consts::E;
use std::fmt::{Debug, Display, Formatter, write};
use std::sync::mpsc::Sender;
use crate::utils::put_blank_line;

pub fn practice() {
    struct A;
    struct Single(A);

    struct SingleGen<T>(T);

    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');

    generic_func();
    generic_impl();
    generic_trait();
    generic_bound();
}

fn generic_bound() {
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    struct P {
        age: i32
    }
    impl Display for P {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, " {}: {} ", "age", self.age)
        }
    }
    printer(P{
        age: 1,
    });

    struct S<T: Display>(T);
    //let s = S(vec![1]); Vector is not a Display
    
    trait HasArea {
        fn area(&self) -> f64;
    }
    #[derive(Debug)]
    struct Rectangle { length: f64, height: f64 }
    #[allow(dead_code)]
    struct Triangle  { length: f64, height: f64 }
    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length  * self.height
        }
    }

    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _trianle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: {:?}", t);
        println!("Dispaly: {}", t);
    }
    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: {:?}", t);
        println!("u: {:?}", u);
    }

    let string = "word";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);

    struct Years(i64);
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    put_blank_line();

    struct Container(i32, i32);
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }
    impl Contains<i32, i32> for Container {
        fn contains(&self, n1: &i32, n2: &i32) -> bool {
            (&self.0 == n1) && (&self.1 == n2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<A, B, C>(container: &C) -> i32
        where C:Contains<A, B> {
        container.last() - container.first()
    }

    let num1 = 3;
    let num2 = 10;
    let container = Container(num1, num2);
    println!("does container contain {} and {}: {}",
        &num1, &num2, container.contains(&num1, &num2));
    println!("fisrt num: {}", container.first());
    println!("last num: {}", container.last());
    println!("difference is {}", difference(&container));

    put_blank_line();

    try_type();
}

fn try_type() {
    /*
    trait Contains {
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    }
    fn difference<C: Contains>(container: &C) -> i32 {
        ...
    }
    */
    struct Container(i32, i32);
    trait Contains {
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, n1: &Self::A, n2: &Self::B) -> bool {
            (&self.0 == n1) && (&self.1 == n2)
        }

        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }
    let n1 = 3;
    let n2 = 10;
    let container = Container(n1, n2);

    println!("Does container contain {} and {}: {}", &n1, &n2,
        container.contains(&n1, &n2));
    println!("The difference is {}", difference(&container));

    put_blank_line();
}

fn generic_trait() {
    struct Empty;
    struct Null;

    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }
    //调用者 U， drop了 T
    impl <T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {

        }
    }

    let empty = Empty;
    let null = Null;
    empty.double_drop(null);

}

fn generic_impl() {
    struct S;
    struct GenericVal<T>(T,);

    impl GenericVal<f32> {}
    impl GenericVal<S> {}
    impl <T> GenericVal<T> {}

    struct Val {
        val: f64
    }

    struct GenVal<T> {
        gen_val: T
    }

    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    impl <T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }
}

fn generic_func() {
    put_blank_line();

    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic::<char>(SGen('a'));
    generic(SGen('c'));

}