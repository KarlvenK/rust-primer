use crate::utils::put_blank_line;

pub fn practice() {
    raii();
    ownership();
    variability();
    borrow();
    life_time();
}

fn life_time() {
    put_blank_line();

    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {}, y is {}", x, y);
    }

    fn failed_borrow<'a>() {
        let _x = 0i32;

        //error. the life time of _x is not long enough.
        //because y live as long as `'a`
       //let y: &'a i32 = &_x;
       //delete `'a` then we can pass the complier
    }
    let (eight, nine) = (8, 9);
    print_refs(&eight, &nine);
    failed_borrow();

    {
        fn print_one<'a>(x: &'a i32) {
            println!("print_one: x is {}", x);
        }
        fn add_one<'a>(x: &'a mut i32) {
            *x += 1;
        }
        fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("print_multi: x is {}, y is {}", x, y);
        }
        fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
            x
        }

        let x= 7;
        let y = 9;
        print_one(&x);
        print_multi(&x, &y);
        let z = pass_x(&x, &y);
        print_one(z);
        let mut t = 3;
        add_one(&mut t);
        print_one(&t);

        /* `'a` must live longer than fn
        这里的 `&String::from("foo")` 将会创建一个 `String` 类型，然后对它取引用。
        数据在离开作用域时删掉，返回一个指向无效数据的引用。
        fn invalid_output<'a>() -> &'a String {
            &String::from("foo")
        }*/
    }
    put_blank_line();

    {
        struct Owner(i32);
        //方法一般是不需要标明生命周期的
        // 因为 self 的生命周期会赋给所有的输出 生命周期参数
        impl Owner {
            fn add_one<'a>(&'a mut self) {
                self.0 += 1;
            }
            fn print<'a>(&'a self) {
                println!("print: {}", self.0);
            }
        }

        let mut owner = Owner(18);
        owner.add_one();
        owner.print();
    }

    {
        #[derive(Debug)]
        struct Borrowed<'a>(&'a i32);
        #[derive(Debug)]
        struct NamedBorrowed<'a> {
            x: &'a i32,
            y: &'a i32,
        }
        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }

        let x = 18;
        let y = 16;
        let single = Borrowed(&x);
        let double = NamedBorrowed {x: &x, y: &y};
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("single is borrowed in {:?}", single);
        println!("double are borrowed in {:?}", double);
        println!("reference is borrowed in {:?}", reference);
        println!("number is not borrowed in {:?}", number);

    }

    {
        #[derive(Debug)]
        struct Borrowed<'a> {
            x: &'a i32,
        }

        impl <'a> Default for Borrowed<'a> {
            fn default() -> Self {
                Self {x: &10,}
            }
        }
        let b: Borrowed = Default::default();
        println!("b is {:?}", b);
    }
    put_blank_line();
    {
        use std::fmt::Debug;
        #[derive(Debug)]
        struct Ref<'a, T: 'a>(&'a T);
        // `Ref` 包含一个指向泛型类型 `T` 的引用，其中 `T` 拥有一个未知的生命周期
        // `'a`。`T` 拥有生命周期限制， `T` 中的任何*引用*都必须比 `'a` 活得更长。另外
        // `Ref` 的生命周期也不能超出 `'a`。
        fn print<T>(t: T) where
            T: Debug {
            println!("`print`: t is {:?}", t);
        }
        // 这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait，并且在 `T` 中的
        // 所有*引用*都必须比 `'a'` 存活时间更长。另外，`'a` 也要比函数活得更长。
        fn print_ref<'a, T>(t: &'a T) where
            T: Debug + 'a {
            println!("`print_ref`: t is {:?}", t);
        }

        let x = 7;
        let ref_x = Ref(&x);
        print_ref(&ref_x);
        print(ref_x);
    }

    {
        //一个较长的生命周期可以强制转成一个较短的生命周期

        // 在这里，Rust 推导了一个尽可能短的生命周期。
        // 然后这两个引用都被强制转成这个生命周期。
        fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
            first * second
        }
        // `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长。
        // 在这里我们我们接受了一个 `&'a i32` 类型并返回一个 `&'b i32` 类型，这是
        // 强制转换得到的结果。
        fn choose_first<'a: 'b, 'b>(first:&'a i32, second:&'b i32) -> &'b i32 {
            first
        }
        let first = 2;
        {
            let second = 3;
            println!("The product is {}", multiply(&first, &second));
            println!("{} is the first", choose_first(&first, &second));
        }
    }

    {
        // 产生一个拥有 `'static` 生命周期的常量。
        static NUM: i32 = 18;

        // 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
        // 而是被强制转换成和输入参数的一样。
        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }

        {
            // 产生一个 `string` 字面量并打印它：
            let static_string = "I'm in read-only memory";
            println!("static_string: {}", static_string);

            // 当 `static_string` 离开作用域时，该引用不能再使用，不过
            // 数据仍然存在于二进制文件里面。
        }

        {
            // 产生一个整型给 `coerce_static` 使用：
            let lifetime_num = 9;

            // 将对 `NUM` 的引用强制转换成 `lifetime_num` 的生命周期：
            let coerced_static = coerce_static(&lifetime_num);

            println!("coerced_static: {}", coerced_static);
        }

        println!("NUM: {} stays accessible!", NUM);

    }

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