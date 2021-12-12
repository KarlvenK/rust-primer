pub mod multi_type;

pub mod vector {
    use crate::utils;
    pub fn test() {
        println!("test over");
    }
    pub fn practice() {
        let mut v1: Vec<i32> = vec![1,2,3];
        let v2 = vec![0; 10];
        println!("{}", v1[0]);

        for i in &v1 {
            print!("{} ", i);
        }
        utils::put_blank_line();
        for i in &v2 {
            print!("{} ", i);
        }
        utils::put_blank_line();

        for i in &mut v1 {
            *i = *i + 1;
            print!("{} ", i);
        }
        utils::put_blank_line();
    }
}

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