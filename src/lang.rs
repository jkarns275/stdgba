use core::panic::PanicInfo;

#[lang = "panic_impl"]
#[no_mangle]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() -> ! { loop {} }

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
