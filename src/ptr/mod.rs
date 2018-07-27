use core::ops::{ Deref, DerefMut, Index, IndexMut };
use core::mem;
use core::intrinsics::{ volatile_store, volatile_load };

#[derive(Clone, Copy)]
pub union Ptr<T: Sized> {
    pub ptr: * const T,
    pub ptr_mut: * mut T,
    pub num: u32,
    pub signed: i32
}

impl<T: Sized> Ptr<T> {

    pub const fn from_u32(i: u32) -> Self { Ptr { num: i } }

    pub const fn from_ptr(ptr: * const T) -> Self { Ptr { ptr: ptr } }

    pub const fn from_mut_ptr(ptr_mut: * mut T) -> Self { Ptr { ptr_mut } }

    pub const fn from_ref(const_ref: &T) -> Self { Ptr { ptr: const_ref as * const T } }

    pub fn from_mut_ref(mut_ref: &mut T) -> Self { Ptr { ptr_mut: mut_ref as * mut T } }

    pub const fn null() -> Self { Ptr { num: 0 } }

    pub const fn transmute<S: Sized>(self) -> Ptr<S> {
        Ptr::<S>::from_u32(self.num)
    }

    pub const fn is_null(&self) -> bool { self.num == 0 }

    pub unsafe fn as_ref(self) -> &'static T { mem::transmute(self.ptr) }

    pub unsafe fn as_mut(mut self) -> &'static mut T { mem::transmute(self.ptr_mut) }

    pub fn offset(mut self, n: i32) -> Self {
        self.signed += n * mem::size_of::<T>() as i32;
        self
    }

    #[inline(always)]
    pub fn volatile_load(&self) -> T {
        unsafe { volatile_load(self.ptr) }
    }

    #[inline(always)]
    pub fn volatile_store(&mut self, dat: T) {
        unsafe { volatile_store(self.ptr_mut, dat); }
    }
}

impl<T: Sized> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { mem::transmute::<* const T, &T>(self.ptr) }
    }
}

impl<T: Sized> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { mem::transmute::<* mut T, &mut T>(self.ptr_mut) }
    }
}

impl<T: Sized, Ind: Sized + Into<i32>> IndexMut<Ind> for Ptr<T> {

    fn index_mut(&mut self, index: Ind) -> &'static mut T {
        let i: i32 = index.into();
        unsafe {
            let x = Ptr::<T>::from_u32(self.num).offset(i);
            x.as_mut()
        }
    }
}

impl<T: Sized, Ind: Sized + Into<i32>> Index<Ind> for Ptr<T> {
    type Output = T;

    fn index(&self, index: Ind) -> &'static T {
        let i: i32 = index.into();
        unsafe {
            let x = Ptr::<T>::from_u32(self.num).offset(i);
            x.as_ref()
        }
    }
}
