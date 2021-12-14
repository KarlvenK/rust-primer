use crate::utils::put_blank_line;
use std::time;

pub fn practice() {
    put_blank_line();

    let mut v1: Vec<i32> = Vec::new();

    let v: Vec<i32> = vec![];
    let v = vec![1, 2, 3];
    let v = vec![0;10];

    let v: Vec<_> = (1..5).collect();
    println!("{:?}", v);

    let a = vec![1, 2, 3];
    assert_eq!(a[1], 2);
    assert_eq!(a.get(1), Some(&2));
    assert_eq!(a.get(3), None);

    let mut v = vec![1, 2, 3];
    for i in &v {
        print!("{} ", *i);
    }
    put_blank_line();
    for i in &mut v {
        *i += 1;
        print!("{} ", *i);
    }
    put_blank_line();
    for i in v {//ownership moved here
        print!("{} ", i);
    }

    put_blank_line();

    let mut v: Vec<usize> = vec![];
    push_1m(&mut v, 5_000_000);
    let mut v: Vec<usize> = vec![];
    v.reserve(5_000_000);
    push_1m(&mut v, 5_000_000);

    put_blank_line();
}

fn push_1m(v: &mut Vec<usize>, total: usize) {
    let e = time::SystemTime::now();
    for i in 1..total {
        v.push(i);
    }
    let ed = time::SystemTime::now();
    println!("time spend: {:?}", ed.duration_since(e).unwrap());
}