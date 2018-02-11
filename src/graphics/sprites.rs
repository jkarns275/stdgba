/// Further documentation sprite related gba things can be found here: https://www.cs.rit.edu/~tjh8300/CowBite/CowBiteSpec.htm#Graphics%20Hardware%20Overview
/// and also here: https://www.coranac.com/tonc/text/regobj.htm


#[derive(Copy, Clone)]
#[repr(u16)]
pub enum SpriteMode {
    /// Enables normal sprite rendering
    Normal      = 0x0000_u16,

    /// Enables alpha blending
    Alpha       = 0x0400_u16,

    /// As per TONC: "Object is part of the object window. The sprite itself isn't rendered, but
    /// serves as a mask for bgs and other sprites. (I think, haven't used it yet)"
    Masked      = 0x0800_u16,

    /// This value is invalid / unused, but is here so it can be used if someone is interested in
    /// testing it.
    Forbidden   = 0x0C00_u16
}

#[derive(Copy, Clone)]
#[repr(u16)]
pub enum AffineMode {
    /// Enables normal affine rendering
    Normal    = 0x0000_u16,

    /// Sprite is an affine sprite and uses the specified affine matrix
    Affine          = 0x0100_u16,

    /// Sprite is hidden
    Disabled        = 0x0200_u16,

    /// Doubles the size of the sprite (I think?)
    Doubled         = 0x0300_u16
}

#[derive(Copy, Clone)]
#[repr(u16)]
pub enum ColorMode {
    _4bpp   = 0x0_u16,
    _8bpp   = 0x2000_u16,
}

/// An enum that represents a sprite's dimensions (width then height). Since there are two attributes that need to be
/// set to set the dimensions, this enum can be converted into a tuple: the first element is the bits to
/// be masked into attribute_0, and the latter into attribute into attribute_1.
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum SpriteDimensions {
    _8x8            = 0x0000_0000_u32,
    _16x16          = 0x0000_4000_u32,
    _32x32          = 0x0000_8000_u32,
    _64x64          = 0x0000_C000_u32,
    _16x8           = 0x4000_0000_u32,
    _32x8           = 0x4000_4000_u32,
    _32x16          = 0x4000_8000_u32,
    _64x32          = 0x4000_C000_u32,
    _8x16           = 0x8000_0000_u32,
    _8x32           = 0x8000_4000_u32,
    _16x32          = 0x8000_8000_u32,
    _32x64          = 0x8000_C000_u32
}

impl SpriteDimensions {

    pub const fn into_tuple(self) -> (u16, u16) {
        ((self as u32 >> 16) as u16, self as u32 as u16)
    }

    /* If you remove the const qualifier, you could do

    pub fn into_tuple(self) -> (u16, u16) {
        mem::transmute(self as u32)
    }

    but I think the const will allow for more optimizations
    */
}

#[derive(Copy, Clone)]
#[repr(u16)]
pub enum SpritePriority {
    Last            = 0x0000_u16,
    Background      = 0x0400_u16,
    Foreground      = 0x0800_u16,
    First           = 0x0C00_u16,
}

#[derive(Copy, Clone)]
pub struct SpriteAttributes {
    /// Attribute 0 (attribute_0)
    a0: u16,

    /// Attribute 1 (attribute_1)
    a1: u16,

    /// Attribute 2 (attribute_2)
    a2: u16,
    /// Filler to make the struct word aligned
    pub filler: u16
}

#[allow(unused)]
impl SpriteAttributes {
    /// All masks used for attribute_0

    /// The 8 bits that are used to set the Y coordinate of the sprite in attribute_0
    const Y_COORD_MASK: u16         = 0x00FF_u16;
    /// The two bits that determine the what affine is used for this sprite; located in attribute_0
    const AFFINE_MODE_MASK: u16     = 0x0300_u16;
    /// The two bits that determine the what special effects are enabled for this sprite; located in
    /// attribute_0
    const SPRITE_MODE_MASK: u16     = 0x0C00_u16;
    /// If this bit is set to 1 in attribute_0, then mosaic effects are enabled
    const MOSAIC_MASK: u16         = 0x1000_u16;
    /// The bit that determine what color mode is used in attribute_0. If it is 0, then it is 4bpp
    /// (16 color), otherwise it is 8bpp (256 colors).
    const COLOR_MODE_MASK: u16       = 0x2000_u16;
    /// The first attribute that determines the dimensions of the sprite, in attribute_0 (first element
    /// in the tuple from SpriteDimensions).
    const SPRITE_SHAPE_MASK: u16     = 0xC000_u16;

    /// All masks used for attribute_1

    /// The 9 (yes 9) bits that are used to set the X coordinate of the sprite in attribute_1
    const X_COORD_MASK: u16          = 0x01FF_u16;
    /// The affine index bits in attribute_1. Should only be set if AFFINE_MODE is set to Affine
    const AFFINE_INDEX_MASK: u16     = 0x3E00_u16;
    /// The bit to be set if this sprite should be horizontally flipped, in attribute_1.
    const HORIZONTAL_FLIP_MASK: u16  = 0x1000_u16;
    /// The bit to be set if this sprite should be vertically flipped, in attribute_1.
    const VERTICAL_FLIP_MASK: u16    = 0x2000_u16;
    /// The second attribute that determines the dimensions of the sprite, in attribute_1 (second
    /// element in the tuple from SpriteDimensions)
    const SPRITE_SIZE_MASK: u16     = 0xC000_u16;

    /// All masks used for attribute_2

    const TILE_INDEX_MASK: u16          = 0x03FF_u16;
    const PRIORITY_MASK: u16            = 0x0C00_u16;
    const PALETTE_BANK_INDEX_MASK: u16  = 0xF000_u16;

    pub fn default() -> Self { SpriteAttributes { a0: 0, a1: 0, a2: 0, filler: 0 } }

    pub fn new( x: u16, y: u16, affine_mode: AffineMode, sprite_mode: SpriteMode,
                dimensions: SpriteDimensions, color_mode: ColorMode, mosaic_enabled: bool,
                horizontal_flipped: bool, vertical_flipped: bool, priority: SpritePriority,
                palette_bank_index: u16, tile_index: u16) -> Self {

        let mut result = Self::default();
        result.set_x(x);
        result.set_y(y);
        result.set_priority(priority);
        result.set_dimensions(dimensions);
        result.set_tile_index(tile_index);
        result.set_color_mode(color_mode);
        result.set_affine_mode(affine_mode);
        result.set_sprite_mode(sprite_mode);
        result.set_mosaic_enabled(mosaic_enabled);
        result.set_vertically_flipped(vertical_flipped);
        result.set_palette_bank_index(palette_bank_index);
        result.set_horizontally_flipped(horizontal_flipped);

        result
    }

    pub fn set_x(&mut self, mut x: u16) {
        x &= Self::X_COORD_MASK;
        self.a1 &= !SpriteAttributes::X_COORD_MASK;
        self.a1 |= x;
    }

    pub fn set_y(&mut self, mut y: u16) {
        y &= Self::Y_COORD_MASK;
        self.a0 &= !SpriteAttributes::Y_COORD_MASK;
        self.a0 |= y;
    }

    pub fn set_priority(&mut self, priority: SpritePriority) {
        let pr = priority as u16;
        self.a2 &= !SpriteAttributes::PRIORITY_MASK;
        self.a2 |= pr;
    }

    pub fn set_dimensions(&mut self, dim: SpriteDimensions) {
        let (width,height) = dim.into_tuple();

        self.a0 &= !SpriteAttributes::SPRITE_SHAPE_MASK;
        self.a0 |= width;

        self.a1 &= !SpriteAttributes::SPRITE_SIZE_MASK;
        self.a1 |= height;
    }

    pub fn set_color_mode(&mut self, color_mode: ColorMode) {
        self.a0 &= !SpriteAttributes::COLOR_MODE_MASK;
        self.a0 |= color_mode as u16;
    }

    pub fn set_affine_mode(&mut self, affine_mode: AffineMode) {
        let am = affine_mode as u16;
        self.a0 &= !SpriteAttributes::AFFINE_MODE_MASK;
        self.a0 |= am;
    }

    pub fn set_sprite_mode(&mut self, sprite_mode: SpriteMode) {
        let sm = sprite_mode as u16;
        self.a0 &= !SpriteAttributes::SPRITE_MODE_MASK;
        self.a0 |= sm;
    }

    pub fn set_mosaic_enabled(&mut self, enabled: bool) {
        let p = (enabled as u16) << 12;
        self.a0 &= !SpriteAttributes::MOSAIC_MASK;
        self.a0 |= p;
    }

    pub fn set_vertically_flipped(&mut self, flipped: bool) {
        let p = (flipped as u16) << 13;
        self.a1 &= !SpriteAttributes::VERTICAL_FLIP_MASK;
        self.a1 |= p;
    }

    pub fn set_horizontally_flipped(&mut self, flipped: bool) {
        let p = (flipped as u16) << 12;
        self.a1 &= !SpriteAttributes::HORIZONTAL_FLIP_MASK;
        self.a1 |= p;
    }

    pub fn set_palette_bank_index(&mut self, mut index: u16) {
        index <<= 12;
        self.a2 &= !SpriteAttributes::PALETTE_BANK_INDEX_MASK;
        self.a2 |= index;
    }

    pub fn set_tile_index(&mut self, mut index: u16) {
        index &= SpriteAttributes::TILE_INDEX_MASK;
        self.a2 &= !SpriteAttributes::TILE_INDEX_MASK;
        self.a2 |= index;
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct SpriteAffine {
    fill0: [u16; 3],
    pa: i16,
    fill1: [u16; 3],
    pb: i16,
    fill2: [u16; 3],
    pc: i16,
    fill3: [u16; 3],
    pd: i16
}
