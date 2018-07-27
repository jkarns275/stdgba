use reg;
use graphics::ColorMode;

/// Represents a Background control object as per http://www.coranac.com/tonc/text/regbg.htm
#[derive(Copy, Clone)]
pub struct BgControl(u16);

impl BgControl {

    /// Returns the n'th background control. There are only 4, so only values of n [0,3]are
    /// valid.
    pub fn get(mut n: u32) -> &'static mut BgControl {
        n &= 3;
        unsafe { reg::REG_BGCNT.transmute::<BgControl>().offset(n as i32).as_mut() }
    }

    pub fn set_priority(&mut self, mut priority: u16) {
        self.0 &= !0b11;
        priority &= 0b11;
        self.0 |= priority;
    }

    pub fn set_character_base_block(&mut self, mut block_n: u16) {
        self.0 &= !0b1100;
        block_n &= 0b11;
        self.0 |= block_n << 2;
    }

    pub fn set_color_mode(&mut self, color_mode: ColorMode) {
        self.0 &= !0x80;
        self.0 |= (color_mode as u16) >> 6;
    }

    pub fn set_mosaic_enabled(&mut self, mosaic: bool) {
        let p = mosaic as u16;
        self.0 &= !0x01000000;
        self.0 |= p << 6;
    }

    pub fn set_affine_wrapping_enabled(&mut self, wrapping: bool) {
        let p = wrapping as u16;
        self.0 &= !0x2000;
        self.0 |= (p << 13);
    }

    pub fn set_screen_base_block(&mut self, mut block_n: u16) {
        block_n &= 0b11111;
        self.0 |= block_n << 8;
    }

    pub fn set_bg_size<Bg: BackgroundSize>(&mut self, bg_size: Bg) {
        self.0 &= 0x3FFF;
        self.0 |= bg_size.into_bg_bits();
    }
}

trait BgSize { fn into_bg_bits(self) -> u16; }

/// Represents a regular background size in terms of tiles (that is, an 8x8 image).
#[repr(u16)]
pub enum RegularBgSize {
    /// 256x256 pixels
    _32x32 = 0x0000,

    /// 512x256 pixels
    _64x32 = 0x4000,

    /// 256x512 pixels
    _32x64 = 0x8000,

    /// 512x512 pixels
    _64x64 = 0xC000,
}

impl BgSize for RegularBgSize { fn into_bg_bits(self) -> u16 { self as u16 } }

#[repr(u16)]
pub enum AffineBgSize {
    _16x16 = 0x0000,
    _32x32 = 0x4000,
    _64x64 = 0x8000,
    _128x128 = 0xC000,
}

impl BgSize for AffineBgSize { fn into_bg_bits(self) -> u16 { self as u16 } }

#[repr(c)]
#[derive(Clone)]
struct BgOffsetInternal { x: i16, y: i16 }

#[derive(Clone)]
/// Represents the offset of a background. Since the x and y fields in BgOffsetInternal are write
/// only, we keep a copy of x and y in this struct (since we can't read from the registers to use
/// normal arithmetic operators like +=).
///
/// The x and y coordinates of the background offset will be `mod mapsize`.
struct BgOffset {
    x: i16,
    y: i16,
    inner: * mut BgOffsetInternal
}

impl BgOffset {
    pub fn get(x: i16, y: i16, mut n: u32) -> BgOffset {
        n &= 3;
        BgOffset {
            x, y,
            inner: reg::REG_BG_OFS.transmute::<BgOffset>().offset(n)
        }
    }

    pub fn set_x(&mut self, x: i16) -> &mut BgOffset {
        self.x = x;
        self.inner.x = x;
        self
    }

    pub fn set_y(&mut self, y: i16) -> &mut BgOffset {
        self.y = y;
        self.inner.y = y;
        self
    }

    pub fn set(&mut self, x: i16, y: i16) -> &mut BgOffset {
        self.x = x;
        self.inner.x = x;
        self.y = y;
        self.inner.y = y;
        self
    }

    pub fn translate(&mut self, x: i16, y: i16) -> &mut BgOffset {
        self.x += x;
        self.y += y;
        self.inner.x = self.x;
        self.inner.y = self.y;
        self
        // self.set_x(self.x + x)
        //     .set_y(self.y + y)
    }

    /// Same as `set_x` except it doesn't return &mut BgOffset, hence nc: 'no-chain'
    pub fn set_x_nc(&mut self, x: i16) {
        self.inner.x = x;
        self.x = x;
    }

    /// Same as `set_y` except it doesn't return &mut BgOffset, hence nc: 'no-chain'
    pub fn set_y_nc(&mut self, y: i16) {
        self.y = y;
        self.inner.y = y;
    }

    /// Same as `set` except it doesn't return &mut BgOffset, hence nc: 'no-chain'
    pub fn set_nc(&mut self, x: i16, y: i16) {
        self.x = x;
        self.inner.x = x;
        self.y = y;
        self.inner.y = y;
    }

    /// Same as `translate` except it doesn't return &mut BgOffset, hence nc: 'no-chain'
    pub fn translate_nc(&mut self, x: i16, y: i16) {
        self.x += x;
        self.y += y;
        self.inner.x = self.x;
        self.inner.y = self.y;
        // self.set_x(self.x + x)
        //     .set_y(self.y + y)
    }
}

/// Holds metadata about a tile at a position. The position of the tile is determined by the
/// TileEntry's location (index) in the Screenblock array.
pub struct TileEntry(u16);

impl TileEntry {

}

/// A `Screenblock` is used to store a tilemap (i.e. indices into a tile set - maps position to a
/// tile + palette entry)
pub type Screenblock = [TileEntry; 0x400];

pub struct Tilemap {
    inner: *mut Screenblock
}

impl Tilemap {
    /// Creates a new Tilemap, without initializing any memory to zero (i.e. do it yourself if you
    /// need to). Since there are 32 screenblocks in total, n = n (mod 32)
    pub fn new(mut n: i32) -> Self {
        n &= 31;
        let inner = reg::VRAM.transmute::<Screenblock>().offset(n);
        Tilemap { inner }
    }

    pub fn entry(&mut self, n: u32) -> &'static mut TileEntry {
        unsafe { self.inner.offset(n).transmute().as_mut() }
    }
}


#[repr(c)]
pub struct BgAffine {
    _padding: [u16; 4],
    dx: i32,
    dy: i32
}