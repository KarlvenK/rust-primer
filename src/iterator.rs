use std::collections::HashMap;
use crate::utils::put_blank_line;

pub fn practice() {
    for i in 1..10 {
        print!("{} ", i);
    }
    put_blank_line();

    let nums = vec![1,2,3,4,5,6,7];
    //nums: Vec 调用 into_iter 来转换成迭代器
    for x in nums {
        print!("{} ", x);
    }
    put_blank_line();

    let v: Vec<_> = (1..20).collect();
    let v = (1..20).collect::<Vec<_>>();
    println!("{:?}", v);

    let m = (1..20).fold(1u64, |mul, x| mul*x);
    println!("{}", m); // = 19!

    put_blank_line();

    let v: Vec<_> = (1..20).filter(|x| x%2 == 0).collect();
    println!("{:?}", v);
    let v: Vec<_> = (1..20).map(|x| x-1).collect();
    println!("{:?}", v);

    let v = vec![1,2,3,4,5,6];
    let v_take = v.iter()
        .cloned()
        .take(3)
        .collect::<Vec<_>>();
    println!("{:?}", v_take);

    let v_skip: Vec<_> = v.iter()
        .cloned()
        .skip(2)
        .collect();
    println!("{:?}", v_skip);

    let names = vec!["a", "b", "c"];
    let scores = vec![1,2,3];
    let score_map: HashMap<_,_> = names.iter()
        .zip(scores.iter())
        .collect();
    println!("{:?}", score_map);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(idx, val)| val)
        .fold(0u64, |sum, acm| sum + acm);
    println!("{}", val);
}