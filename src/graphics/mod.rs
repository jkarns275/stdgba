use core::mem;
use core::default::Default;
use core::intrinsics::volatile_store;
use reg;
use ptr::Ptr;
use collections::StaticArr;

pub mod sprites;
pub use self::sprites::*;


#[derive(Clone, Copy)]
#[repr(u8)]
pub enum VideoMode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
    Mode3 = 3,
    Mode4 = 4,
    Mode5 = 5,
}

impl VideoMode {
    const MASK: u8 = 0b0000_0111_u8;

    pub fn set(self, val: u32) -> u32 {
        let p = self as u32;
        (val & !(Self::MASK as u32)) | p
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FrameBufferStart {
    /// The FrameBuffer should start at the address 0x06000000
    Base     = 0b0000_0000,
    /// The FrameBuffer should start at the address 0x0600A000
    Offset   = 0b0001_0000,
}

impl FrameBufferStart {
    const MASK: u8 = 0b0001_0000_u8;

    pub fn set(self, val: u32) -> u32 {
        let p = self as u32;
        (val & !(Self::MASK as u32)) | p
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum SpriteStorageMode {
    _2D = 0b0000_0000,
    _1D = 0b0100_0000
}

impl SpriteStorageMode {
    const MASK: u8 = 0b0100_0000_u8;

    pub fn set(self, val: u32) -> u32 {
        let p = self as u32;
        (val & !(Self::MASK as u32)) | p
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum HBlankProcessing {
    None  = 0b0000_0000_u8,
    Force = 0b0010_0000_u8,
}

impl HBlankProcessing {
    const MASK: u8 = 0b0010_0000_u8;

    pub fn set(self, val: u32) -> u32 {
        let p = self as u32;
        (val & !(Self::MASK as u32)) | p
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DisplayState {
    Blank   = 0b1000_0000_u8,
    On      = 0b0000_0000_u8,
}

impl DisplayState {
    const MASK: u8 = 0b1000_0000_u8;

    pub fn set(self, val: u32) -> u32 {
        let p = self as u32;
        (val & !(Self::MASK as u32)) | p
    }
}

pub struct GraphicsMode {
    pub vm: VideoMode,
    pub frame_buffer_start: FrameBufferStart,
    pub hblank_policy: HBlankProcessing,
    pub sprite_storage_mode: SpriteStorageMode,
    pub display_state: DisplayState,
    pub bg0_enabled: bool,
    pub bg1_enabled: bool,
    pub bg2_enabled: bool,
    pub bg3_enabled: bool,
    pub sprites_enabled: bool,
    pub window0_enabled: bool,
    pub window1_enabled: bool,
    pub sprite_windows_enabled: bool,
}

impl GraphicsMode {

    const BG0_MASK: u16 = 0x0100;
    const BG1_MASK: u16 = 0x0200;
    const BG2_MASK: u16 = 0x0400;
    const BG3_MASK: u16 = 0x0800;
    const SPRITES_MASK: u16 = 0x1000;
    const WINDOW0_MASK: u16 = 0x2000;
    const WINDOW1_MASK: u16 = 0x2000;
    const SPRITE_WINDOWS_MASK: u16 = 0x2000;


    pub fn current() -> GraphicsMode {
        unsafe { GraphicsMode::from_u16(mem::transmute(*reg::REG_GRAPHICS_MODE)) }
    }

    pub fn from_u16(n: u16) -> GraphicsMode {
        GraphicsMode {
            vm: VideoMode::Mode0,
            frame_buffer_start:     FrameBufferStart::Base,
            hblank_policy:          HBlankProcessing::None,
            sprite_storage_mode:    SpriteStorageMode::_2D,
            display_state:          DisplayState::On,
            bg0_enabled:            (n & GraphicsMode::BG0_MASK) != 0,
            bg1_enabled:            (n & GraphicsMode::BG1_MASK) != 0,
            bg2_enabled:            (n & GraphicsMode::BG2_MASK) != 0,
            bg3_enabled:            (n & GraphicsMode::BG3_MASK) != 0,
            sprites_enabled:        (n & GraphicsMode::SPRITES_MASK) != 0,
            window0_enabled:        (n & GraphicsMode::WINDOW0_MASK) != 0,
            window1_enabled:        (n & GraphicsMode::WINDOW1_MASK) != 0,
            sprite_windows_enabled: (n & GraphicsMode::SPRITE_WINDOWS_MASK) != 0,
        }
    }

    pub fn set(&self) {
        let mut reg = 0u16;
        reg |= self.vm as u16;
        reg |= self.frame_buffer_start as u16;
        reg |= self.hblank_policy as u16;
        reg |= self.sprite_windows_enabled as u16;
        reg |= self.display_state as u16;
        reg |= self.sprite_storage_mode as u16;

        let bg0 = if self.bg0_enabled { GraphicsMode::BG0_MASK } else { 0 };
        let bg1 = if self.bg1_enabled { GraphicsMode::BG1_MASK } else { 0 };
        let bg2 = if self.bg2_enabled { GraphicsMode::BG2_MASK } else { 0 };
        let bg3 = if self.bg3_enabled { GraphicsMode::BG3_MASK } else { 0 };
        let sprites = if self.sprites_enabled { GraphicsMode::SPRITES_MASK } else { 0 };
        let window0 = if self.window0_enabled { GraphicsMode::WINDOW0_MASK } else { 0 };
        let window1= if self.window1_enabled { GraphicsMode::WINDOW1_MASK } else { 0 };
        let sprite_windows = if self.sprite_windows_enabled { GraphicsMode::SPRITE_WINDOWS_MASK} else { 0 };

        reg |= bg0 | bg1 | bg2 | bg3 | sprites | window0 | window1 | sprite_windows;

        // The * mut _ is to prevent a weird warning, that may be a bug in rustc
        // when using reg::REG_GRAPHICS_MODE.ptr_mut, a warning as thrown that says:
        //
        // --> src\graphics\mod.rs:164:33
        //    |
        //    164 |         unsafe { volatile_store(reg::REG_GRAPHICS_MODE.ptr_mut, reg) }
        //    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //    |
        //    = note: #[warn(const_err)] on by default

        unsafe { volatile_store(reg::REG_GRAPHICS_MODE.num as * mut _, reg) }
    }

}

impl Default for GraphicsMode {
    fn default() -> Self {
        GraphicsMode::from_u16(0)
    }
}