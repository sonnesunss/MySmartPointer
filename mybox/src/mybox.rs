use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr;


#[allow(dead_code)]
#[derive(Debug)]
pub struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    pub(crate) fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        let size = layout.size();

        let ptr =  if size == 0 {
            ptr::NonNull::<T>::dangling().as_ptr()
        } else {
            let ptr = unsafe {
                alloc(layout)
            } as *mut T;
            if ptr.is_null() {
                panic!("Allocation failed");
            }

            unsafe {
                ptr::write(ptr, value);
            }
            ptr
        };

        MyBox { ptr }
    }
}

// 使得结构体具备智能指针的行为
// 即使用*获取到底层数据引用的行为， 这里与普通的用在指针上面的 *解引用运算符 还是有区别的，需要注意下！！！
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.ptr
        }
    }
}

// 资源释放
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();

        if layout.size() > 0 {
            unsafe {
                // 丢弃值
                ptr::drop_in_place(self.ptr);
                // 释放layout内存布局形式的，从self.ptr地址处开始的堆内存空间
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_zst() {
        let my_box = MyBox::new(42);
        assert_eq!(*my_box, 42);

        let mut my_box = MyBox::new(String::from("hello"));
        *my_box = String::from("world");
        assert_eq!(*my_box, "world");
    }

    #[test]
    fn test_zst() {
        let my_box = MyBox::new(());
        assert_eq!(*my_box, ());

        #[derive(Debug, PartialEq)]
        struct Empty;
        let my_box = MyBox::new(Empty);
        assert_eq!(*my_box, Empty);
    }
}
