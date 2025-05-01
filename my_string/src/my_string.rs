use std::ops::Deref;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MyString {
    inner: Rc<String>,
}

#[allow(dead_code)]
impl MyString {
    pub fn new(s: &str) -> Self {
        MyString {
            inner: Rc::new(s.to_string()),
        }
    }

    pub fn ref_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

impl Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for MyString {
    fn clone(&self) -> Self {
        MyString {
            inner: Rc::clone(&self.inner),
        }
    }
}
