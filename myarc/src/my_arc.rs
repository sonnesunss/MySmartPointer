use core::fmt;
use std::{ops::Deref, ptr::NonNull, sync::atomic::AtomicUsize};

#[allow(dead_code)]
pub struct MyArc<T> {
    inner: NonNull<MyArcInner<T>>,
}

#[allow(dead_code)]
pub struct MyArcInner<T> {
    ref_count: AtomicUsize,
    data: T,
}

#[allow(dead_code)]
impl<T> MyArc<T> {
    pub fn new(data: T) -> Self {
        let boxed = Box::new(MyArcInner {
            ref_count: AtomicUsize::new(1),
            data,
        });

        MyArc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) },
        }
    }

    pub fn strong_count(&self) -> usize {
        unsafe {
            self.inner
                .as_ref()
                .ref_count
                .load(std::sync::atomic::Ordering::SeqCst)
        }
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };

        inner
            .ref_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        MyArc { inner: self.inner }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.inner.as_ref().data }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };

        if inner
            .ref_count
            .fetch_sub(1, std::sync::atomic::Ordering::Release)
            == 1
        {
            std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.inner.as_ptr()));
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for MyArc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyArc")
            .field("data", &**self)
            .field("ref_count", &Self::strong_count(self))
            .finish()
    }
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new_arc() {
        let arc = MyArc::new(42);
        assert_eq!(*arc, 42);
        assert_eq!(arc.strong_count(), 1);
    }

    #[test]
    fn test_clone_increases_count() {
        let arc = MyArc::new(99);
        let arc2 = arc.clone();
        assert_eq!(arc.strong_count(), 2);
        assert_eq!(arc2.strong_count(), 2);
        assert_eq!(*arc2, 99);
    }

    #[test]
    fn test_drop_decreases_count() {
        let arc = MyArc::new(7);
        let arc2 = arc.clone();
        assert_eq!(arc.strong_count(), 2);

        drop(arc2);
        assert_eq!(arc.strong_count(), 1);
    }

    #[test]
    fn test_multithreaded_usage() {
        let arc = MyArc::new(1234);
        let arc2 = arc.clone();

        let handle = thread::spawn(move || {
            assert_eq!(*arc2, 1234);
            assert_eq!(arc2.strong_count(), 2);
        });

        handle.join().unwrap();
        assert_eq!(arc.strong_count(), 1);
    }
}
