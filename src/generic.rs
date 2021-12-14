use crate::utils::put_blank_line;

pub fn practice() {
    let a = Some(100.111f32);
    let a:Option<f32> = Some(100.111);
    let b:Option<f32> = Some(1000.111f32);
    let c:Option<f64> = Some(100.11);
    let d:Option<f64> = Some(100.11f64);

    println!("{} {} {} {}", a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap());
    put_blank_line();
    generic_fn();
}

fn generic_fn() {
    use std::ops::Add;
    fn add<T: Add<T, Output=T>>(a: T, b:T) -> T {
        a + b
    }

    println!("100 + 1 = {}", add(100i32, 1i32));
    println!("100.11 + 10.11 = {}", add(100.11, 10.11));

    put_blank_line();

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, p: Point) -> Point {
            Point {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }

    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 2, y: 1};
    print!("{:?}  + {:?} = ", p1, p2);
    println!("{:?}", add(p1, p2));

    put_blank_line();

    struct_be_generic();
}

fn struct_be_generic() {
    use std::ops::Add;
    #[derive(Debug)]
    struct Point<T: Add<T, Output=T>> {
        x: T,
        y: T,
    }

    impl<T: Add<T, Output=T>> Add for Point<T> {
        type Output = Point<T>;
        fn add(self, p: Point<T>) ->Point<T> {
            Point {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }

    fn add<T: Add<T, Output=T>>(a: T, b: T) -> T {
        a + b
    }
    let p1 = Point{x:1.11f32, y:2.22f32};
    let p2 = Point{x:1.11f32, y:2.22f32};
    println!("{:?}", add(p1, p2));
    put_blank_line();
}