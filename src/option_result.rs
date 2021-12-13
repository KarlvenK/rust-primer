use std::num::ParseIntError;
use crate::utils::put_blank_line;

pub fn practice() {
    opt();
    rst();

}

fn rst() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
    match double_number("aaa") {
        Ok(n) => assert_eq!(n, 20),
        Err(e) => println!("Error: {:?}", e),
    }
    put_blank_line();
}

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    //2 * number_str.parse::<i32>().unwrap()
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn opt() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found"),
        Some(i) => println!("File extension: {}", &file_name[i+1 ..])
    }
    put_blank_line();
}
fn find(hay: &str, needle: char) -> Option<usize> {
    for (offset, c) in hay.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}