#![feature(i128_type, asm, lang_items, core, core_intrinsics, const_fn, untagged_unions, arbitrary_self_types)]
#![no_std]
#![feature(use_extern_macros)]

pub extern crate gbaimg;
pub use gbaimg::{ img_as_palleted_sprite_8bpp, img_as_palleted_sprite_4bpp };

mod lang;
pub use lang::*;

#[macro_use]
pub mod interrupt;
pub mod reg;
pub mod ptr;
pub mod input;
pub mod alloc;
pub mod boxed;
pub mod collections;
pub mod mem;
pub mod graphics;

