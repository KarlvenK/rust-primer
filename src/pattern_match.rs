use crate::utils::put_blank_line;

pub fn practice() {
    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };

    let d_west = Direction::West;
    let d_str = match d_west {
        Direction::East => "East",
        Direction::North | Direction::South => {
            panic!("South or North");
        },
        _ => "West",
    };
    println!("{}", d_str);
    put_blank_line();

    destructure();
    pattern();
    range();
    reference();
    binding();
}

fn binding() {
    let x = 1u32;
    match x {
        e @ 1..=5 | e @ 10..=15 => println!("get {}", e),
        _ => {}
    }

    put_blank_line();
}

fn reference() {
    let mut x = 5;
    match x {
        ref mut mr => println!("mut ref :{}", mr),
    }
    let ref mut mrx = x;
    put_blank_line();
}

fn range() {
    let x = 1;
    match x {
        1 ..= 10 => println!("1 to 10"),
        _ => println!("other"),
    }

    let c = 'w';

    match c {
        'a' ..= 'z' => println!("lowercase"),
        'A' ..= 'Z' => println!("uppercase"),
        _ => println!("other"),
    }

    let x = 1;
    match x {
        1 | 2 => println!("1 or 2"),
        _ => println!("other"),
    }
}

fn destructure() {
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let action = Action::Say("Hello Boy".to_string());
    // let action = Action::ChangeColorRGB(255, 255, 255);
    match action {
        Action::Say(s) => {
            println!("{}", s);
        },
        Action::MoveTo(x, y) => {
            println!("point from (0, 0) move to ({}, {})", x, y);
        },
        Action::ChangeColorRGB(r,g,_) => {
            println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",r, g);
        }
    }

    struct Point {
        x: i64,
        y: i64,
    }
    let point = Point{
        x: 0,
        y: 0,
    };
    match point {
        Point{
            x, y
        } => println!("({}, {})", x, y),
    }
    //or
    match point {
        Point {
            x: x1,
            y: y1,
        } => println!("({}, {})", x1, y1),
    }

    let point = Point{
        x: 0,
        y: 0,
    };
    match point {
        Point{y,..} => println!("y is {}", y),
    }


    let tuple: (u32, String) = (5, String::from("five"));

    let (x, s) = tuple;
    //String????????????Copy??? ??????tuple?????????move??????????????????????????????????????????
    //println!("Tuple is : {:?}", tuple);

    let tuple = (5, String::from("five"));
    //String???_???????????? i32?????????Copy?????????tuple?????????move
    let (x, _) = tuple;
    println!("{:?}", tuple);

    put_blank_line();
}

fn pattern() {
    let x = 1;
    let c = 'c';
    match c {
        x => println!("[x in match]: {}, [c]: {}", x, c),
    }
    println!("outer [x]:{}", x);
    put_blank_line();

    let x = 4;
    let y = false;
    match x {
        4|5 if y => println!("yes"),
        _ => println!("no"),
    }
    put_blank_line();
}
