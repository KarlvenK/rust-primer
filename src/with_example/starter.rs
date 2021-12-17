use crate::utils::put_blank_line;
use super::output;
use super::enumerate;
use super::type_transform;

pub fn start() {
    output::practice();
    enumerate::practice();
    type_transform::practice();
    try_iterator();
    try_destructure();
}

fn try_destructure() {
    put_blank_line();
    let reference = &10086;
    match reference {
        /*
                `&i32`
                 | |
                `&val`
         */
        &val => println!("got a value {:?}", val),
    }

    match *reference {
        val => println!("got a value {:?}", val),
    }

    put_blank_line();

    let not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("got a reference to a value {:?}", r),
    }
    println!("mut_value: {:?}", mut_value);
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("we added 10. `mut_value`: {:?}", m);
        },
    }
}

fn try_iterator() {
    let names = vec!["a", "b", "c"];

    for name in names.iter() {
        match name  {
            &"a" => println!("!"),
            _ => println!("no"),
        }
    }
    put_blank_line();

    for name in names.into_iter() {
        match name {
            "a" => println!("!"),
            _ => ()
        }
    }
    println!("names destructed");
    //println!("{:?}", names);

    let mut names = vec![1, 2, 3];
    for name in names.iter_mut() {
        match name {
            &mut 1 => println!("!"),
            _ => ()
        }
    }
    println!("{:?}", names);

}