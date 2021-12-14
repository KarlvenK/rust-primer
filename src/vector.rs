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