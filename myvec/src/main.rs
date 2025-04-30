mod my_vec;

use my_vec::MyVec;

fn main() {
    println!("Hello, world!");

    let mut v1 = MyVec::new();
    v1.push(1);
    v1.push(1);
    v1.push(1);
    v1.push(1);

    println!("{:?}", v1);
}
