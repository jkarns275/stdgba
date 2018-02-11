use ptr::Ptr;
use alloc::{ alloc, free };
use core::mem;
use core::ops::{ Deref, DerefMut, Drop };
use core::intrinsics::volatile_store;

pub struct Box<T: Sized> {
    inner: Ptr<T>
}

impl<T: Sized> Box<T> {
    pub fn new(item: T) -> Self {
        unsafe {
            let inner = alloc::<T>(1);
            volatile_store(inner.ptr_mut, item);
            Box { inner }
        }
    }
}

impl<T: Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { mem::transmute::<* const T, &T>(self.inner.ptr) }
    }
}

impl<T: Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { mem::transmute::<* mut T, &mut T>(self.inner.ptr_mut) }
    }
}

impl<T: Sized> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe { free(&mut self.inner) }
    }
}