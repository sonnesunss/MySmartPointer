use std::alloc::{Layout, alloc, dealloc};
use std::ops::{Deref, DerefMut};
use std::ptr;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

#[allow(dead_code)]
impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.reallocate();
        }

        unsafe {
            ptr::write(self.ptr.add(self.len), value);
            self.len += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn reallocate(&mut self) {
        let new_capacity = if self.capacity == 0 {
            8
        } else {
            self.capacity * 2
        };

        let new_ptr = unsafe {
            let layout = Layout::array::<T>(new_capacity).unwrap();
            let new_ptr = alloc(layout) as *mut T;

            if !self.ptr.is_null() {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                dealloc(
                    self.ptr as *mut u8,
                    Layout::array::<T>(self.capacity).unwrap(),
                );
            }

            new_ptr
        };

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                for i in 0..self.len {
                    // drop_in_place函数用于在指定内存地址上手动调用某个值的析构函数，
                    // 而不移动或销毁内存本身
                    // 对于POD类型没有必要，其他类型就需要否则可能引起内存泄露
                    // 例如当T = String时，如果不在释放堆内存空间之前释放掉String类型占用的内存空间
                    // 则那部分空间就会被泄露掉
                    // 对于原始的POD类型则无需调用此函数
                    ptr::drop_in_place(self.ptr.add(i));
                }

                dealloc(
                    self.ptr as *mut u8,
                    Layout::array::<T>(self.capacity).unwrap(),
                );
            }
        }
    }
}
