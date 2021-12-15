use crate::utils::put_blank_line;

pub fn practice() {
    put_blank_line();
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click{
            x: i64,
            y: i64,
        }
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::Paste(s) => println!("pasted \"{}\"", s),
            WebEvent::KeyPress(c) => println!("pressed \"{}\"", c),
            WebEvent::Click {x, y} => {
                println!("clicked at x = {}, y = {}", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_string());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    put_blank_line();

    enum ThisIsALongNameOfEnumerate {
        Add,
        Subtract,
    }

    type Operation = ThisIsALongNameOfEnumerate;

    let x = Operation::Add;

    impl Operation {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    println!("1 + 2 = {}", x.run(1, 2));
}