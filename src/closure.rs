use crate::utils::put_blank_line;

pub fn practice() {
    let plus_one = |x:i32| x + 1;
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut res: i32 = x;
        res += 1;
        res += 1;
        res
    };
    assert_eq!(4, plus_two(2));


    fn plus_one_v1 (x: i32) -> i32 {x + 1}
    let plus_one_v2 = |x:i32| -> i32 {x + 1};
    let plus_one_v3 = |x: i32|        x + 1;

    let num = 5;
    let plus_num = |x: i32| x + num;
    assert_eq!(10, plus_num(5));

    /* error
    let mut num = 5;
    let plus_num = |x: i32| x + num;
    let y = &mut num;
    */

    //ok
    let mut num = 5;
    {
        let plus_num = |x: i32| x + num;
    } // plus_num goes out of scope, borrow of num ends
    let y = &mut num;
    *y += 1;
    println!("{}", *y);

    /*error
    let nums = vec![1,2,3];
    {
        let takes_nums = || nums;
    }
    println!("{:?}", nums);
     */
    put_blank_line();

    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);

    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
        println!("{}\n", num);
    }
    assert_eq!(5, num);

    let s = "hello";
    let c = ||{println!("{:?}", s)};
    c();
    c();
    println!("{:?}", s);

    let s = "hello";
    let c = move || {println!("{:?}", s)};
    c();
    c();
    println!("{:?}", s);

}