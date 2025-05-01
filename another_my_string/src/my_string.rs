use std::ops::Deref;
use std::ptr;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MyString {
    ptr: *mut u8,          //  可变u8字节类型，即字节地址
    len: usize,            // 字符串长度
    capacity: usize,       // 分配的内存容量
    ref_count: *mut usize, // 引用计数
}

#[allow(dead_code)]
impl MyString {
    pub fn new(s: &str) -> Self {
        // 计算需要的大小
        let len = s.len();
        let capacity = len + 1; // 加一个结尾处的空字节大小
        // 分配堆内存空间
        let layout = std::alloc::Layout::array::<u8>(capacity).expect("allocation failed");
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut u8;

        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        // 复制字符串数据到堆中去
        unsafe {
            ptr::copy_nonoverlapping(s.as_ptr(), ptr, len);
            *ptr.add(len) = 0; // 添加结尾空字节
        }

        // 分配引用计数
        let ref_count = unsafe {
            let layout = std::alloc::Layout::new::<usize>();
            let ptr = std::alloc::alloc(layout) as *mut usize;

            if ptr.is_null() {
                panic!("Memory allocation failed for ref_count");
            }

            *ptr = 1; // 初始引用计数为1,这里是裸指针所以使用*表示访问底层数据
            ptr
        };

        MyString {
            ptr: ptr,
            len: len,
            capacity: capacity,
            ref_count: ref_count,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn ref_count(&self) -> usize {
        unsafe { *self.ref_count }
    }

    // 作为&str 返回字符串
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.len)) }
    }
}

impl Clone for MyString {
    fn clone(&self) -> Self {
        unsafe {
            *self.ref_count += 1;
        }

        MyString {
            ptr: self.ptr,
            len: self.len,
            capacity: self.capacity,
            ref_count: self.ref_count,
        }
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("开始释放资源");
        unsafe {
            *self.ref_count -= 1;

            if *self.ref_count == 0 {
                let layout =
                    std::alloc::Layout::array::<u8>(self.capacity).expect("Invalid layout");
                std::alloc::dealloc(self.ptr, layout);

                let layout = std::alloc::Layout::new::<usize>();
                std::alloc::dealloc(self.ref_count as *mut u8, layout);
            }
        }
        println!("释放资源完毕!");
    }
}

impl Deref for MyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
