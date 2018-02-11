use ptr::Ptr;
use core::mem::{ size_of };

pub unsafe fn memcpy<T: Sized>(dst: Ptr<T>, src: Ptr<T>, items: u32) -> Ptr<T> where Ptr<T>: Clone + Copy {
    if items == 0 || dst.is_null() || src.is_null() {
        return dst;
    }

    let mut count: u32;
    let mut dst16: Ptr<u16>;
    let mut src8 = src.transmute::<u8>();

    let mut len = items * (size_of::<T>() as u32);

    let mut p = Ptr::<u32>::from_u32(0x03000004);
    *p = 0;

    // Copy data 4 words at a time - except for the tail, which is 0, 1, 2, or 3 words
     if (src.num | dst.num) & 4 == 0 && len >= 4 {
         *p = 1;
         let src32 = src.transmute::<u32>();
         let mut dst32 = dst.transmute::<u32>();

         count = len / 4;
         let mut tail_words = count & 3;
         count /= 4;

         while tail_words != 0 {
             *dst32 = *src32;
             dst32.offset(1);
             src32.offset(1);
             tail_words-= 1;
         }

         while count != 0 {
             *dst32 = *src32;
             dst32.offset(1);
             src32.offset(1);

             *dst32 = *src32;
             dst32.offset(1);
             src32.offset(1);

             *dst32 = *src32;
             dst32.offset(1);
             src32.offset(1);

             *dst32 = *src32;
             dst32.offset(1);
             src32.offset(1);

             count -= 1;
         }

         *p = 2;

         len &= 3;
         if len == 0 {
             return dst;
         }

         src8 = src32.transmute();
         dst16 = dst32.transmute();
     } else {
         let dst_offset = dst.num & 1;
         dst16 = Ptr::<u16>::from_u32(dst.num - dst_offset);

         if dst_offset != 0 {
             *dst16 = (*dst16 & 0xFF) | ((*src8 as u16) << 8);
             src8.offset(1);
             dst16.offset(1);
             len -= 1;
             if len == 0 {
                 return dst;
             }
         }
     }

    count = len / 2;

    *p = 4;

    while count != 0 {
        *dst16 = src8[0] as u16 | ((src8[1] as u16) << 8);
        dst16.offset(1);
        src8.offset(2);
        len -= 1;
        count -= 1;
    }

    *p = 5;

    if len & 1 != 0 {
        *dst16 = (*dst16 & !0xFF) | *src8 as u16;
    }

    *p = 6;

    return dst
}