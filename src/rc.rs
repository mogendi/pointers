use crate::cell::Cell;
use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

struct RcInner<T> {
    value: T,
    count: Cell<usize>,
}

// !Sync is implied by Cell on RcInner
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _mapper: PhantomData<RcInner<T>>,
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let current_count = unsafe { (&*self.inner.as_ptr()).count.get() };
        unsafe { (&*self.inner.as_ptr()).count.set(current_count + 1) };
        Rc {
            inner: self.inner,
            _mapper: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(&*self.inner.as_ptr()).value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let count = unsafe { (&*self.inner.as_ptr()).count.get() };
        if count == 1 {
            unsafe { drop(self.inner.as_ref()) };
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            unsafe { (&*self.inner.as_ptr()).count.set(count - 1) };
        }
    }
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner_box = Box::new(RcInner {
            value: value,
            count: Cell::new(0),
        });
        // SAFETY: Box::into_raw is guaranteed to return a ptr
        // with the default allocator
        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner_box)) },
            _mapper: PhantomData,
        }
    }
}
