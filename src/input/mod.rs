use reg::REG_KEY_INPUT;
use core::mem;
use core::ops::Deref;

#[derive(Copy, Clone)]
pub struct InputState(u16);

impl InputState {
    pub fn current() -> Self {
        InputState(*REG_KEY_INPUT)
    }

    pub fn all_keys_down<T: Into<KeySet>>(self, keys: T) -> bool {
        let keys = keys.into();
        !self.0 & keys.0 == keys.0
    }

    pub fn any_keys_down<T: Into<KeySet>>(self, keys: T) -> bool {
        let keys = keys.into();
        !self.0 & keys.0 != 0
    }

    pub fn pressed_keys(self) -> KeySet {
        KeySet(!self.0)
    }

    pub fn key_down(self, key: Key) -> bool {
        !self.0 & key as u16 == key as u16
    }
}

/// A set of keys
#[derive(Copy, Clone)]
pub struct KeySet(pub u16);

const ALL_KEYS: KeySet = KeySet(0x03FF);

impl KeySet {
    pub fn empty() -> KeySet {
        KeySet(0)
    }

    pub fn all() -> KeySet {
        ALL_KEYS
    }

    pub fn contains(self, key: Key) -> bool {
        self.0 & *key != 0
    }

    pub fn add(mut self, key: Key) -> Self {
        self.0 |= *key;
        self
    }
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum Key {
    A =      0x0001,
    B =      0x0002,
    Select = 0x0004,
    Start  = 0x0008,
    Right  = 0x0010,
    Left   = 0x0020,
    Up     = 0x0040,
    Down   = 0x0080,
    R      = 0x0100,
    L      = 0x0200,
}

impl Deref for Key {
    type Target = u16;

    fn deref(&self) -> &u16 {
        unsafe { mem::transmute::<&Key, &u16>(self) }
    }
}

impl Into<KeySet> for Key {
    fn into(self) -> KeySet {
        KeySet(*self)
    }
}