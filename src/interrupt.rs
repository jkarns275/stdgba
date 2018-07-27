macro_rules! interrupt {
    ($x:expr) => (unsafe { asm!(concat!("swi ", $x) : : : "r0", "r1", "r2", "r3" : "volatile"); })
}
