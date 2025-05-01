use my_string::MyString;

mod my_string;

fn main() {
    println!("Hello, world!");
    let s1 = MyString::new("Hello My String smart pointer!");
    let s2 = s1.clone();
    let s3 = s1.clone();

    println!("{:?} - {:?}", s1, s1.ref_count());
    println!("{:?} - {:?}", s3, s3.ref_count());
    println!("{:?} - {:?}", s2, s2.ref_count());
}
