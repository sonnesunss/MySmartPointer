// 实现自己的String智能指针
mod my_string;

use my_string::MyString;

fn main() {
    let s1 = MyString::new("Hello MyString ptr");
    println!("s1: {:?}, ref_count: {:?}", s1, s1.ref_count());

    let _s2 = s1.clone();
    let _s3 = s1.clone();

    println!("s1: {:?}, ref_count: {:?}", s1, s1.ref_count());
}
