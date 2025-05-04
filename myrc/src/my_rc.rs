/*

# 实现自己的简单Rc引用计数指针

+ 可以突破rust的单一所有权机制，允许同一个值有多个所有者，智能指针通过跟踪值的引用
次数管理内存，当引用计数降为0时释放资源

+ 使具备智能指针的一些行为
+ myrc本身应该具备创建、克隆、计数方面的功能
+ 单线程下

*/

use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;

struct MyRcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

pub struct MyRc<T> {
    inner: NonNull<MyRcInner<T>>,
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.inner.as_ptr();
            let count = (*inner).ref_count.get();
            println!("Dropping MyRc, ptr: {:p}, ref_count: {}", inner, count);

            // Decrease reference count
            if count == 0 {
                eprintln!("Error: ref_count is already 0 for ptr: {:p}", inner);
                return;
            }
            (*inner).ref_count.set(count - 1);

            // Only free if ref_count reaches 0
            if count == 0 {
                println!("Freeing MyRcInner, ptr: {:p}", inner);
                // Explicitly drop the value
                std::ptr::drop_in_place(&mut (*inner).value);
                // Free the MyRcInner memory
                drop(Box::from_raw(inner));
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.inner.as_ptr()).value }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let inner = self.inner.as_ptr();
            (*inner).ref_count.set((*inner).ref_count.get() + 1);
            println!(
                "Cloning MyRc, ptr: {:p}, new ref_count: {}",
                inner,
                (*inner).ref_count.get()
            );
            MyRc { inner: self.inner }
        }
    }
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(MyRcInner {
            value,
            ref_count: 1.into(),
        });
        let inner_ptr = NonNull::new(Box::into_raw(inner)).expect("Failed to allocate");
        println!("Creating MyRc, ptr: {:p}, ref_count: 1", inner_ptr.as_ptr());
        MyRc { inner: inner_ptr }
    }

    pub fn get_ref_count(&self) -> usize {
        unsafe { (*self.inner.as_ptr()).ref_count.get() }
    }
}

unsafe impl<T: Send + Sync> Send for MyRc<T> {}
unsafe impl<T: Send + Sync> Sync for MyRc<T> {}
