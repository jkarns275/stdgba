use core::ops::{ Index, IndexMut };
use core::mem::size_of;

use ptr::Ptr;
use alloc::{ alloc, free };

pub struct Arr<T: Sized> {
    ptr: Ptr<T>,
    len: u32,
}

impl<T: Sized> Arr<T> {
    pub fn new(len: u32) -> Arr<T> {
        unsafe {
            let data_ptr: Ptr<T> = alloc::<T>(len);
            Arr {
                ptr: data_ptr,
                len: len * size_of::<T>() as u32,
            }
        }
    }

    pub fn len(&self) -> u32 { self.len }

    pub fn free(mut self) {
        unsafe {
            free(&mut self.ptr);
        }
    }
}

impl<Item, Ind> Index<Ind> for Arr<Item>
    where Item: Sized,
          Ind: Sized + Into<u32> {
    type Output = Item;

    fn index(&self, index: Ind) -> &Item {
        let i: u32 = index.into();
        unsafe {
            let x = Ptr::<Item>::from_u32(self.ptr.num + i * size_of::<Item>() as u32);
            let y: &mut Item;
            asm!("mov $0, $1"
            : "=&r" (y)
            : "r"(x)
            :
            : );
            y
        }
    }
}

impl<Item, Ind> IndexMut<Ind> for Arr<Item>
    where Item: Sized,
          Ind: Sized + Into<u32> {

    fn index_mut(&mut self, index: Ind) -> &mut Item {
        let i: u32 = index.into();
        unsafe {
            let x = Ptr::<Item>::from_u32(self.ptr.num + i * size_of::<Item>() as u32);
            let y: &mut Item;
            asm!("mov $0, $1"
            : "=&r" (y)
            : "r"(x)
            :
            : );
            y
        }
    }
}