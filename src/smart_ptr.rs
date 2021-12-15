use std::cell::{Cell, RefCell};
use std::thread;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::channel;
use std::collections::HashMap;

use crate::utils::put_blank_line;

pub fn practice() {
    use_rc();
    //use_arc();
    //mutex();
    cell();
}

fn circular_reference() {
    put_blank_line();
    struct Owner {
        name: String,
        toys: RefCell<Vec<Weak<Toy>>>,
    }

    struct Toy {
        id: i32,
        owner: Rc<Owner>,
    }

    {
        let toy_owner: Rc<Owner> = Rc::new(
            Owner {
                name: "toy man".to_string(),
                toys: RefCell::new(Vec::new()),
            }
        );

        let toy1 = Rc::new(Toy{
            id: 1,
            owner: toy_owner.clone(),
        });
        let toy2 = Rc::new(Toy {
            id: 2,
            owner: toy_owner.clone(),
        });

        toy_owner.toys.borrow_mut().push(Rc::downgrade(&toy1));
        toy_owner.toys.borrow_mut().push(Rc::downgrade(&toy2));

        for toy_opt in toy_owner.toys.borrow().iter() {
            let toy = toy_opt.upgrade().unwrap();
            println!("toy {} owned by {}", toy.id, toy_owner.name);
        }
    }

}

fn cell() {
    put_blank_line();
    let c = Cell::new(5);
    let five = c.get();
    println!("{}", five);
    c.set(10);
    println!("{}", c.get());

    put_blank_line();

    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("1", 1);
    shared_map.borrow_mut().insert("2", 2);

    let c = RefCell::new(5);
    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();
    //let m = c.borrow_mut();   //causes a panic

    let c = RefCell::new(5);
    let five = c.into_inner();

    circular_reference();

}

fn mutex() {
    const N: usize = 10;

    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }
    rx.recv().unwrap();

    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}

fn use_rc() {
    put_blank_line();
    //immutable
    let five = Rc::new(5);
    let five2 = five.clone();
    let five3 = five.clone();
    println!("{}", five2);
    let weak_five = Rc::downgrade(&five);
    let strong_five: Option<Rc<_>> = weak_five.upgrade();

}

fn use_arc() {
    put_blank_line();
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);

    for i in 0..10 {
        let child_numbers = shared_numbers.clone();
        thread::spawn(move || {
            let local_numbers = &child_numbers[..];
            //work with the local numbers
            for j in 0..10 {
                print!("{} ", local_numbers[i*j]);
            }
            put_blank_line();
        });
    }

    put_blank_line();

    struct Owner {
        name: String,
    }
    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    let gadget_owner: Rc<Owner> = Rc::new(
        Owner {
            name: String::from("Gadget Man"),
        }
    );

    let gadget1 = Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: gadget_owner.clone(),
    };

    drop(gadget_owner);
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    put_blank_line();
}