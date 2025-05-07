use my_arc::MyArc;

mod my_arc;

fn main() {
    println!("Hello, world!");

    let a = MyArc::new(vec![1, 2, 3]);
    let b = a.clone();

    println!("{:?}", a); // MyArc { data: [1, 2, 3], ref_count: 2 }
    println!("{:?}", b); // MyArc { data: [1, 2, 3], ref_count: 2 }

    drop(a);
    println!("{:?}", b); // ref_count = 1
}
