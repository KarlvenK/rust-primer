use std::any::TypeId;
use std::fmt::{DebugList, format, Formatter};
use std::num::ParseIntError;
use crate::utils::put_blank_line;
use std::error;
use std::error::Error;
use std::fmt;

pub fn practice() {
    use_option();
    combinator();
    use_result();
    result_from_option();
}

fn result_from_option() {
    put_blank_line();

    {
        fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
            vec.first().map(|first| {
                first.parse::<i32>().map(|n| 2 * n)
            })
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers));

        println!("The first doubled is {:?}", double_first(empty));
        // Error 1: the input vector is empty

        println!("The first doubled is {:?}", double_first(strings));
        // Error 2: the element doesn't parse to a number
    }

    put_blank_line();

    {
        type Result<T> = std::result::Result<T, DoubleError>;
        #[derive(Debug, Clone)]
        struct DoubleError;

        impl fmt::Display for DoubleError {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        impl error::Error for DoubleError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                None
            }
        }

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            vec.first()
                .ok_or(DoubleError)
                .and_then(|s| {
                    s.parse::<i32>()
                        .map_err(|_| DoubleError)
                        .map(|i| 2 * i)
                })
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n)  => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

fn use_result() {
    put_blank_line();

    fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
        let first_number = first_number_str.parse::<i32>().unwrap();
        let second_number = second_number_str.parse::<i32>().unwrap();
        first_number * second_number
    }

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);
    //error panic
    //let tt = multiply("t", "2");
    {
        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            match first_number_str.parse::<i32>() {
                Ok(first_number) => {
                    match second_number_str.parse::<i32>() {
                        Ok(second_num) => {
                            Ok(first_number * second_num)
                        },
                        Err(e) => Err(e),
                    }
                },
                Err(e) => Err(e),
            }
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => {
                    println!("n is {}", n);
                },
                Err(e) => println!("Error: {}", e),
            }
        }

        let twenty = multiply("2", "10");
        print(twenty);

        let err = multiply("t", "2");
        print(err);
    }

    {
        fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
            first_num_str.parse::<i32>()
                .and_then(|first_num| {
                    second_num_str.parse::<i32>()
                        .map(|second_num| first_num * second_num)
            })
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        // 这种情况下仍然会给出正确的答案。
        let twenty = multiply("10", "2");
        print(twenty);

        // 这种情况下就会提供一条更有用的错误信息。
        let tt = multiply("t", "2");
        print(tt);
    }
}

fn combinator() {
    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }
    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)|Cooked(food))
        //Some(Chopped) changed into Some(Cooked)
    }

    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm. I love {:?}", food),
            None => println!("I get nothing to eat"),
        }
    }
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = Some(Food::Potato);
    let no_food = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);
    let cooked_nothing = process(no_food);
    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
    eat(cooked_nothing);

    put_blank_line();
    {
        #[derive(Debug)]
        enum Food {
            CordonBlue,
            Steak,
            Sushi,
        }
        #[derive(Debug)]
        enum Day {
            Monday,
            Tuesday,
            Wendesday,
        }

        fn have_ingredients(food: Food) -> Option<Food> {
            match food {
                Food::Sushi => None,// we dont have the ingredients to make sushi
                _ => Some(food),
            }
        }

        fn have_recipe(food: Food) -> Option<Food> {
            match food {
                Food::CordonBlue => None, //we dont have CordonBlue
                _ => Some(food),
            }
        }

        fn cookable_v1(food: Food) -> Option<Food> {
            match have_ingredients(food) {
                None => None,
                Some(food) => match have_recipe(food) {
                    None => None,
                    Some(food) => Some(food),
                }
            }
        }

        fn cookable_v2(food : Food) -> Option<Food> {
            have_ingredients(food).and_then(have_recipe)
        }

        fn eat(food: Food, day: Day) {
            match cookable_v2(food) {
                Some(food) => println!("yay! On{:?}, we get to eat {:?}", day, food),
                None => println!("oh no"),
            }
        }

        let (cordon_blue, steak, sushi) = (Food::CordonBlue, Food::Steak, Food::Sushi);
        eat(cordon_blue, Day::Tuesday);
        eat(steak, Day::Monday);
        eat(sushi, Day::Wendesday);
    }
}

fn use_option() {
    fn give_commoner(gift: Option<&str>) {
        match gift  {
            Some("snake") => println!("Yuck! Im throwing that snake"),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No gift?"),
        }
    }

    fn give_princess(gift: Option<&str>) {
        ///unwrap: if get None then panic
        let inside = gift.unwrap();
        if inside == "snake" {
            panic!("AAAAAAAAA");
        }
        println!("I love {}s", inside);
    }

    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing: Option<&str> = None;
    give_princess(bird);
    //panic because unwrap get None
    //give_princess(nothing);
    put_blank_line();

    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        let next_age: u8 = current_age?;
        // if current_age is None, then return None
        // if current_age is Some, then return u8 in the Option to next_age
        Some(format!("Next year I will be {}", next_age))
    }

    let a = next_birthday(Some(1));
    //panic None
    //let b = next_birthday(None);
    println!("{}", a.unwrap());

    put_blank_line();

    struct Person {
        job: Option<Job>,
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }

    impl Person {
        fn work_phone_area_code(&self) -> Option<u8> {
            self.job?.phone_number?.area_code
        }
    }
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}