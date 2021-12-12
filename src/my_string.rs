use crate::utils::put_blank_line;

pub fn practice() {
    str();
    let x = "messi".to_string();
    println!("{}", x);
    for i in x.as_bytes() {
        print!("{} ", i);
    }
    put_blank_line();
    for i in x.chars() {
        print!("{} ",i );
    }
    put_blank_line();

    format_();
}

fn format_() {
    let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm⾼的⼤胖⼦",81, "wayslog", width=4, height=178);
    println!("{}",s);

}

fn str() {
    put_blank_line();
    let x: &'static str = "hello";

    let mut y: String = x.to_string();
    println!("{}", y);
    y.push_str(" world");
    println!("{}", y);
    put_blank_line();

    let s = "aaaaaa".to_string();
    use_str(&*s);

}

fn use_str(s: &str) {
    println!("i am: {}", s);
}