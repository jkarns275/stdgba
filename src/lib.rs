#![feature(i128_type, asm, lang_items, core, core_intrinsics, const_fn, untagged_unions, arbitrary_self_types)]
#![no_std]

mod lang;
pub use lang::*;

pub mod reg;
pub mod ptr;
pub mod input;
pub mod alloc;
pub mod boxed;
pub mod collections;
pub mod mem;
pub mod graphics;
