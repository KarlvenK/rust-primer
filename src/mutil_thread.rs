use std::{thread, time};
use std::sync::{mpsc, Mutex};
use std::sync::Arc;
use std::sync::Condvar;
use std::rc::Rc;
use crate::utils::put_blank_line;

pub fn practice() {
    //creat_thread();
    send_message();
    shared_ram();
    _sync();
}

fn _sync() {
    //Condvar 需要和 Mutex 一同使用，因为有 Mutex 保护， Condvar 并发才是安全的。
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("notify main thread");
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
}

fn shared_ram() {
    static VAR: i32 = 6;
    let new_thread = thread::spawn(move || {
        println!("static value inner : {}", VAR);
    });
    new_thread.join().unwrap();
    println!("static value outer : {}", VAR);

    put_blank_line();

    static mut VAR_MUT: i32 = 6;
    let newthread = thread::spawn(move || {
        unsafe {
            println!("static value inner : {}", VAR_MUT);
            VAR_MUT += 1;
        }
    });
    newthread.join().unwrap();
    unsafe {
        println!("static value outer : {}", VAR_MUT);
    }

    put_blank_line();

    let var: Arc<i32> = Arc::new(5);
    let share_var = var.clone();

    let new_thread = thread::spawn(move || {
        println!("share value in new thread: {}, address: {:p}", share_var, &*share_var);
    });
    new_thread.join().unwrap();
    println!("share value in main thread: {}, address: {:p}", var, &*var);

    put_blank_line();
}

fn send_message() {
    put_blank_line();

    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) =
        mpsc::channel();

    thread::spawn(move || {
       tx.send(1).unwrap();
    });
    println!("reveice {}", rx.recv().unwrap());

    put_blank_line();

    //asynchronous
    const THREAD_COUNT: i32 = 2;
    {
        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) =
            mpsc::channel();
        for id in 0..THREAD_COUNT {
            let thread_tx = tx.clone();
            thread::spawn(move || {
                thread_tx.send(id + 1).unwrap();
                println!("send {}", id + 1);
            });
        }

        thread::sleep(time::Duration::from_millis(5));
        println!("wake up!");
        for _ in 0..THREAD_COUNT {
            println!("receive {}", rx.recv().unwrap());
        }
    }

    put_blank_line();

    //synchronous
    {
        let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) =
            mpsc::sync_channel(0);

        let new_thread = thread::spawn(move || {
            println!("before send");
            tx.send(0).unwrap();
            println!("after send");
        });

        println!("before sleep");
        thread::sleep(time::Duration::from_millis(10));
        println!("after sleep");
        println!("receive {}", rx.recv().unwrap());
        new_thread.join().unwrap();
    }

    put_blank_line();
}

fn creat_thread() {
    put_blank_line();

    let new_thread = thread::spawn(move || {
        say_something(1);
    });
    new_thread.join().unwrap();

    let new_thread_result = thread::Builder::new()
        .name("thread1".to_string())
        .stack_size(4*1024*1024).spawn(move|| {
        say_something(1)
    });
    new_thread_result.unwrap().join().unwrap();

    put_blank_line();
    /* dead loop. dangerous
    let new_thread = thread::spawn(move|| {
        loop {
            say_something(0);
        }
    });
    new_thread.join().unwrap();
     */
    //create a thread
    let new_thread = thread::spawn(move|| {
        //create another thread
        thread::spawn(move|| {
            loop {
                thread::sleep(time::Duration::from_millis(1));
                say_something(10086);
            }
        });
    });

    new_thread.join().unwrap();
    println!("Child thread is finish!");
    thread::sleep(time::Duration::from_millis(8));

    put_blank_line();
}

fn say_something(x: i32) {
    println!("I am a new thread{}.", x);
}
