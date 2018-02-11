use core;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_args: &core::fmt::Arguments,
                    _file: &str,
                    _line: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() -> ! { loop {} }

// I'm not 100% sure what this function does, but references to it are compiled
// into the program by the Rust compiler. I think it would be called in the case
// of a program panic.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { loop {} }
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() { loop {} }

#[no_mangle]
pub extern "C" fn __multi3(a: i32, b: i32) -> i32 { a * b }

#[no_mangle]
pub extern "C" fn __udivti3(a: u32, b: u32) -> u32 { a / b }

#[no_mangle]
pub extern "C" fn __umodti3(a: u32, b: u32) -> u32 { a % b }
