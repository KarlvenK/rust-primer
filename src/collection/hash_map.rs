use crate::utils::put_blank_line;
use std::collections::HashMap;

pub fn practice() {
    let mut come_from = HashMap::new();
    come_from.insert("a", "aa");
    come_from.insert("fasd", "fasd");
    come_from.insert("fa", "fa");

    if !come_from.contains_key("f") {
        println!("no f");
    }
    put_blank_line();

    come_from.remove("a");

    let who = ["a", "fa", "fasd"];
    for person in &who {
        match come_from.get(person) {
            Some(location) => println!("{}: {}", person, location),
            None => println!("{} not exists", person),
        }
    }
    put_blank_line();

    entry();

}

fn entry() {
    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_eq!(letters.get(&'y'), None);
}