use crate::mybox::MyBox;

mod mybox;

fn main() {
    println!("Hello, world!");

    let box1 = MyBox::new(123);
    let box2 = MyBox::new("Hello mybox smartpointer".to_string());

    println!("{}", *box1);
    println!("{}", *box2);
}
