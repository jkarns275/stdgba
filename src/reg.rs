use ptr::Ptr;

pub const REG_GRAPHICS_MODE: Ptr<u16> = unsafe { Ptr::from_u32(0x04000000) };
pub const REG_BG_AFFINE: Ptr<u16> =     unsafe { Ptr::from_u32(0x04000000) };
pub const REG_VCOUNT: Ptr<u16> =        unsafe { Ptr::from_u32(0x04000006) };
pub const REG_BGCNT: Ptr<u16> =         unsafe { Ptr::from_u32(0x04000008) };
pub const REG_BG_OFS: Ptr<u16> =        unsafe { Ptr::from_u32(0x04000010) };
pub const REG_BG_VOFS: Ptr<u16> =       unsafe { Ptr::from_u32(0x04000012) };
pub const REG_DATA_IN0: Ptr<u16> =      unsafe { Ptr::from_u32(0x04000120) };
pub const REG_DATA_IN1: Ptr<u16> =      unsafe { Ptr::from_u32(0x04000122) };
pub const REG_DATA_IN2: Ptr<u16> =      unsafe { Ptr::from_u32(0x04000124) };
pub const REG_DATA_IN3: Ptr<u16> =      unsafe { Ptr::from_u32(0x04000126) };
pub const REG_SIOCNT: Ptr<u16> =        unsafe { Ptr::from_u32(0x04000128) };
pub const REG_DATA_OUT: Ptr<u16> =      unsafe { Ptr::from_u32(0x0400012A) };
pub const REG_KEY_INPUT: Ptr<u16> =     unsafe { Ptr::from_u32(0x04000130) };
pub const REG_RCNT: Ptr<u16> =          unsafe { Ptr::from_u32(0x04000134) };
pub const REG_IE: Ptr<u16> =            unsafe { Ptr::from_u32(0x04000200) };
pub const REG_IME: Ptr<u16> =           unsafe { Ptr::from_u32(0x04000208) };

pub const VRAM: Ptr<u16> =              unsafe { Ptr::from_u32(0x06000000) };
pub const OAM: Ptr<u32> =               unsafe { Ptr::from_u32(0x07000000) };
