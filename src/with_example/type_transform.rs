use crate::utils::put_blank_line;

pub fn practice() {
    from_into();
    try_from_try_into();
    to_string_from_string();
}

fn to_string_from_string() {
    use std::string::ToString;
    use std::str::FromStr;

    struct Circle {
        radius: i32,
    }
    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle {
        radius: 6,
    };
    println!("{}", circle.to_string());


    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);

    put_blank_line();
}

fn try_from_try_into() {
    use std::convert::TryFrom;
    use std::convert::TryInto;
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));

}

fn from_into() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{}", my_string);

    use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(n: i32) -> Self {
            Number {
                value: n,
            }
        }
    }

    let num = Number::from(1);
    println!("{:?}", num);

    let int = 5;
    let num: Number = int.into();
    //if you impl from, then you get a into() for free
    println!("{:?}", num);

    put_blank_line();
}