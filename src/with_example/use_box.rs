use crate::utils::put_blank_line;

pub fn practice() {
    put_blank_line();

    let x = [0u8; 10];
    print_len(x);

    let x = Box::new([0u8; 4 * 1024 * 1024]);
    print_box_len(&x);
    println!("{}", x.len());

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: Option<Box<Node>>,
    }

    let node = Node {
        value: 1,
        parent: Some(Box::new(Node {
            value: 0,
            parent: None,
        }))
    };

    println!("{}", std::mem::size_of::<Node>());
    println!("{:?}", node);



}

fn print_box_len(arr: &Box<[u8; 4 * 1024 * 1024]>) {
    println!("{}", arr.len());
}

fn print_len(arr: [u8; 10]) { //arr is Copied from original [u8; 10]
    println!("{}", arr.len());
}