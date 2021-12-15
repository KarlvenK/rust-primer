pub mod multi_type;
pub mod my_string;
pub mod ownership;
pub mod option_result;
pub mod trait_;
pub mod generic;
pub mod vector;
pub mod pattern_match;
pub mod func;
pub mod closure;
pub mod collection;
pub mod iterator;

pub mod utils {
    pub fn put_blank_line() {
        println!();
    }
}

pub mod circle {
    use crate::utils::put_blank_line;

    pub fn practice() {
        for x in 0..5 {
            print!("{} ", x);
        }
        put_blank_line();
        for (i, j) in (5..10).enumerate() {
            println!("i = {}, j = {}", i, j);
        }
        put_blank_line();

        let lines = "abcdef\ngggasfsdf\nfasdff\nfasdfsaf".lines();
        for (linenum, line) in lines.enumerate() {
            println!("{}:{}", linenum, line);
        }

        'outer: for x in 0..10 {
            'iner: for y in 0..10 {
                if x % 2 == 0 {
                    continue 'outer;
                }
                if y % 2 == 0 {
                    continue 'iner;
                }
                println!("x: {}, y: {}", x, y);
            }
        }
    }
}
