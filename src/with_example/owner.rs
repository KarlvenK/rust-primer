use crate::utils::put_blank_line;

pub fn practice() {
    raii();
    ownership();
    variability();
    borrow();
}

fn borrow() {
    fn eat_box_i32(boxed_i32: Box<i32>) { //get the ownership, use it, destroy it
        println!("Destroying box that contains {}", boxed_i32);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(9_i32);
    let stacked_i32 = 8_i32;
    borrow_i32(&stacked_i32);
    borrow_i32(&boxed_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        //error! borrow used after
        //eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);

    put_blank_line();

    #[derive(Copy, Clone)]
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} edtion", book.title, book.year);
    }
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrorwed {} - {} edition", book.title, book.year);
    }

    let immutabook = Book {
        author: "ta-ta-kai",
        title: "aaaaa",
        year: 1979,
    };

    let mut mutabook = immutabook;
    borrow_book(&immutabook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    //error
    //new_edition(&mut immutabook);
    let c = 'Q';
    let ref ref_c1 = c;
    //equal to
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    #[derive(Clone, Copy)]
    struct Point{
        x: i32,
        y: i32,
    }
    let point = Point {
        x: 0,
        y: 0,
    };
    let _copy_of_x = {
        let Point{x: ref ref_to_x, y: _} = point;
        *ref_to_x
    };
    //mut copy
    let mut mutable_point = point;
    {
        let Point{x: _, y: ref mut mut_ref_to_y} = mutable_point;
        *mut_ref_to_y = 1;
    }
    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(6u32), 3u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}

fn partial_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };
    let Person{name, ref age} = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    //error! person is partially moved
    //println!("The person struct is {:?}", person);

    // we can not use `person`, but `person.age` is not moved,
    // so we can still use it
    println!("The person's age from person struct is {}", person.age);
}

fn variability() {
    put_blank_line();

    let immutable_box = Box::new(7u32);
    println!("immutable_box contains {}", immutable_box);
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 9;
    println!("mutable_box now contains {}", mutable_box);

    put_blank_line();

    partial_move();

    put_blank_line();
}

fn ownership() {
    put_blank_line();
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    println!("a contains: {}", a);

    //move a to b
    let b = a;
    //this func get the ownership of ram which b points to
    destroyed_box(b);

}

fn destroyed_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    //c destroyed here
}

fn raii() {
    {
        fn create_box() {
            let _box1 = Box::new(3i32);
            //destroyed here
        }
        let _box2 = Box::new(5i32);

        {
            let _box3 = Box::new(4i32);
            //destroyed here
        }

        for _ in 0u32..1_000 {
            create_box();
        }
        //_box2 destroyed here
    }
    struct ToDrop;
    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let x = ToDrop;
    println!("Made a ToDrop");
}