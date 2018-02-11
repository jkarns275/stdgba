use core::ops::{ Index, IndexMut };
use core::mem::transmute;
use ptr::Ptr;

pub struct StaticArr<T: Sized> where Ptr<T>: Clone + Copy {
    ptr: Ptr<T>,
    len: u32,
}

unsafe impl<T: Clone + Copy> Send for StaticArr<T> {}
unsafe impl<T: Clone + Copy> Sync for StaticArr<T> {}

impl<T: Sized + Clone + Copy> StaticArr<T> {
    pub const fn new(ptr: Ptr<T>, len: u32) -> StaticArr<T> {
        StaticArr { ptr, len }
    }

    pub fn len(&self) -> u32 { self.len }

    pub fn zero(&self) -> u32 {
        panic!()
    }

    pub fn as_ptr(&self) -> Ptr<T> {
        self.ptr
    }
}

impl<Item, Ind> Index<Ind> for StaticArr<Item>
    where Item: Sized + Clone + Copy,
          Ind: Sized + Into<i32> {
    type Output = Item;

    fn index(&self, index: Ind) -> &'static Item {
        unsafe {
            let p = self.ptr.clone();
            p.offset(index.into());
            transmute(p)
        }
    }
}

impl<Item, Ind> IndexMut<Ind> for StaticArr<Item>
    where Item: Sized + Clone + Copy,
          Ind: Sized + Into<i32> {

    fn index_mut(&mut self, index: Ind) -> &'static mut Item {
        unsafe {
            let p = self.ptr.clone();
            p.offset(index.into());
            transmute(p)
        }
    }
}