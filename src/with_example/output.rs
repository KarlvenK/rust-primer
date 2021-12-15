use std::fmt;
use std::fmt::{Formatter, write};
use crate::utils::put_blank_line;

pub fn practice() {
    struct Person <'a>{
        name: &'a str,
    }

    impl fmt::Display for Person <'_>{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    println!("{}", Person{
       name: "tt",
    });

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    let minmax = MinMax(0, 1);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    put_blank_line();

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f,"]")
        }
    }
    println!("{}", List(vec![1, 2, 3]));

    put_blank_line();

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl fmt::Display for City {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
            let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name, self.lat.abs(), lat_c, self.lat.abs(), lon_c)
        }
    }

    for city in [
        City{name: "a", lat: 123.0, lon: 12.0},
        City{name: "b", lat: 1.1, lon: 3.2},
    ].iter() {
        println!("{}", *city);
    }


}