use my_rc::MyRc;

mod my_rc;

fn main() {
    println!("Current scope a, a > b");

    let rc1 = MyRc::new("Hello MyRc".to_string());
    println!("value: {}, ref count: {}", *rc1, rc1.get_ref_count());

    // clone
    let rc2 = rc1.clone();
    println!("value: {}, ref count: {}", *rc2, rc2.get_ref_count());

    {
        let rc3 = rc1.clone();
        println!(
            "current scope b: value: {}, ref count: {}",
            *rc3,
            rc3.get_ref_count()
        );
    }

    println!("value: {}, ref count: {}", *rc1, rc1.get_ref_count());
}

#[cfg(test)]
mod tests {
    use crate::my_rc::MyRc;
    #[test]
    fn test_my_rc_basic() {
        let rc = MyRc::new(42);
        assert_eq!(*rc, 42);
        assert_eq!(rc.get_ref_count(), 1);
    }

    #[test]
    fn test_my_rc_clone() {
        let rc1 = MyRc::new(String::from("hello"));
        let rc2 = rc1.clone();
        assert_eq!(*rc1, "hello");
        assert_eq!(*rc2, "hello");
        assert_eq!(rc1.get_ref_count(), 2);
        assert_eq!(rc2.get_ref_count(), 2);
    }

    #[test]
    fn test_my_rc_drop() {
        let rc1 = MyRc::new(100);
        {
            let rc2 = rc1.clone();
            assert_eq!(rc1.get_ref_count(), 2);
            assert_eq!(rc2.get_ref_count(), 2);
        } // rc2 在这里被 drop
        assert_eq!(rc1.get_ref_count(), 1);
    }
}
