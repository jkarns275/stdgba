/// This is a very simple memory allocator that works with the slow RAM offered by the GBA. It has
/// not been extensively tested, and it probably isn't a great idea to use in the same way a
/// memory allocator would be used on a non-embedded system. The best use is to assign memory
/// chunks of memory that will remain there for the entire lifetime of the program; memory can be
/// freed, however doing so will lead to segmentation that may not be recoverable.
///
/// Writing beyond the size of a memory chunk prevent future allocation or deallocation from working
/// properly.
use core::{ mem, iter };
use core::option::*;

use ptr::Ptr;


// first 8 bytes are used to point to free / used lists
const RAM_START:    u32 = 0x02000010;
const RAM_END:      u32 = 0x0203FFFF;

const FREE_HEAD_LOC: u32 = 0x02000000;
const USED_HEAD_LOC: u32 = 0x02000004;

const BLOCK_SIZE_SHIFT: u32 = 7;
const BLOCK_SIZE: u32 = 1 << BLOCK_SIZE_SHIFT;

/// A meta data struct that is found before every pointer returned by the allocator. It is used to link
/// free regions together, and stores information about possible extra length in an allocation
#[derive(Copy, Clone)]
struct Chunk {
    /// Points to the next Chunk in the list
    next: Ptr<Chunk>,
    /// Points to the previous Chunk in the list!
    prev: Ptr<Chunk>,
    /// Size of a sector, not including the space kept up by the Chunk struct
    len: u32,
    /// Used for debugging...
    magic: u32
}

impl Chunk {

    #[inline(always)]
    pub unsafe fn get_free_head() -> Ptr<Ptr<Chunk>> {
        mem::transmute(FREE_HEAD_LOC)
    }

    /// Should only be used when the current head is null!
    pub unsafe fn set_free_head(ptr: Ptr<Chunk>) {
        let current_head: * mut u32 = mem::transmute(FREE_HEAD_LOC);
        *current_head = ptr.num;
    }

    #[inline(always)]
    pub unsafe fn get_used_head() -> Ptr<Ptr<Chunk>> {
        mem::transmute(USED_HEAD_LOC)
    }

    /// Should only be used when the current head is null!
    pub unsafe fn set_used_head(ptr: Ptr<Chunk>) {
        let current_head: * mut u32 = mem::transmute(USED_HEAD_LOC);
        *current_head = ptr.num;
    }

    pub const fn of_size(size: u32) -> Chunk {
        Chunk {
            next: Ptr::null(),
            prev: Ptr::null(),
            len: size,
            magic: 0xDEADBABE
        }
    }

    pub unsafe fn initialize(&mut self) {
        self.magic = 0xCAFEBABE;
        self.next = Ptr::null();
        self.prev = Ptr::null();
        self.len = 0;
    }

    /// Attempts to concatenate two Chunks. The memory address of self should be less than other,
    /// even if they cannot be concatenated. This function will deinitialize other, so all data it
    /// contains will be lost.
    pub unsafe fn try_concatenate(mut self: Ptr<Chunk>, mut other: Ptr<Chunk>) -> bool {
        // If self ends right before other
        if self.after().num == other.num {
            (*self).len += (*other).len + mem::size_of::<Chunk>() as u32;
            (*self).next = (*other).next;
            (*(*self).next).prev = self;
            (*other).deinitialize();
            true
        } else {
            false
        }
    }

    /// Call drop... Mostly for debugging to see deinitialized chunks in memory
    pub unsafe fn deinitialize(&mut self) {
        self.magic = 0xDEADBEEF;
        self.next.num = 0;
        self.prev.num = 0;
        self.len = 0;
    }

    pub unsafe fn append_to_used(mut ptr: Ptr<Chunk>) {
        let head_ptr = Chunk::get_used_head();
        let head: Ptr<Chunk> = *head_ptr;
        if ! head.is_null() {
            // Sort the pointers!
            let mut current: Ptr<Chunk> = head;
            while !(*current).next.is_null() && current.num < ptr.num {
                current = (*current).next;
            }
            let current_prev: Ptr<Chunk> = current.prev;
            (*current).prev = ptr;
            (*ptr).prev = current_prev;
            (*ptr).next = current;
        } else {
            Chunk::set_used_head(ptr);
        }
    }

    pub unsafe fn append_to_free(mut ptr: Ptr<Chunk>) {
        let mut head = Chunk::get_free_head();
        if ! head.is_null() {
            // Sort the pointers and concatenate the given ptr if any adjacent blocks are found.
            let mut current: Ptr<Chunk> = *head;
            if current.num > ptr.num {
                (*head).num = ptr.num;
                (*ptr).next.num = current.num;
                (*ptr).prev = Ptr::null();
            } else {
                while !(*current).next.is_null() && current.num < ptr.num {
                    current = (*current).next;
                }
            }

            let mut current_prev: Ptr<Chunk> = current.prev;

            // Link the chunks together so that ptr is in the middle
            (*current).prev = ptr;
            (*ptr).prev = current_prev;
            (*ptr).next = current;
            if ! current_prev.is_null() {
                (*current_prev).next = ptr;
                // Try to concatenate the previous chunk and ptr
                if current_prev.try_concatenate(ptr) {
                    // The two were successfully concatenated so try appending the new chunk with
                    // the following chunk in the list.
                    // Result doesn't matter here
                    let _ = current_prev.try_concatenate(current);
                }
            } else { // Try to concatenate ptr and the following chunk
                let _ = ptr.try_concatenate(current);
            }
        } else {
            Chunk::set_free_head(ptr);
        }
    }

    /// after returns a pointer to the first byte after the data section
    pub unsafe fn after(&self) -> Ptr<u8> {
        let len = self.len;
        let mut self_ptr = Ptr::<Chunk>::from_ref(self);
        self_ptr.num += len + mem::size_of::<Chunk>() as u32;
        self_ptr.transmute::<u8>()
    }

    /// get_data_ptr returns a pointer to the beginning of the data section after the chunk.
    #[inline(always)]
    pub unsafe fn get_data_ptr<T: Sized>(&self) -> Ptr<T> {
        let mut ptr = Ptr::<Chunk>::from_ref(self).transmute::<T>();
        ptr.num += mem::size_of::<Chunk>() as u32;
        ptr
    }

    #[inline(always)]
    pub unsafe fn as_gba_ptr(&mut self) -> Ptr<Chunk> { Ptr::from_mut_ref(self) }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> * const Chunk { mem::transmute::<&Chunk, * const Chunk>(self) }

    #[inline(always)]
    pub unsafe fn as_ptr_mut(&mut self) -> * mut Chunk { mem::transmute(self) }

    /// Remove this chunk from the list it is contained in.
    pub unsafe fn remove_from_free_list(&mut self) {
        let mut head = Chunk::get_free_head();

        let (next_null, prev_null) = (self.next.is_null(), self.prev.is_null());

        // Current node is:
        match (next_null, prev_null) {
            // Only member in the list
            (true, true) => {
                Chunk::set_free_head(Ptr::null());
            },
            // End of the list
            (true, false) => {
                (*self.prev).next = Ptr::null();
            },
            // Head of list
            (false, true) => {
                *head = self.next;
                (*self.next).prev = Ptr::null();
            },
            // Somewhere in the middle
            (false, false) => {
                let mut next = self.next;
                *next.prev = *self.prev;
                (*self.prev).next = next;
            }
        }

        self.next = Ptr::<Chunk>::null();
        self.prev = Ptr::<Chunk>::null();
    }

    pub unsafe fn remove_from_used_list(&mut self) {
        let mut head = Chunk::get_used_head();

        let (next_null, prev_null) = (self.next.is_null(), self.prev.is_null());

        // Current node is:
        match (next_null, prev_null) {
            // Only member in the list
            (true, true) => {
                Chunk::set_used_head(Ptr::null());
            },
            // End of the list
            (true, false) => {
                (*self.prev).next = Ptr::null();
            },
            // Head of list
            (false, true) => {
                *head = self.next;
                (*self.next).prev = Ptr::null();
            },
            // Somewhere in the middle
            (false, false) => {
                let mut next = self.next;
                *next.prev = *self.prev;
                (*self.prev).next = next;
            }
        }

        self.next = Ptr::<Chunk>::null();
        self.prev = Ptr::<Chunk>::null();
    }

    #[inline(always)]
    pub fn try_alloc(&mut self, buf_len: u32) -> Ptr<Chunk> {
        // If there this block is too small...
         if self.len <= buf_len {
            return Ptr::<Chunk>::null();
        }
        let num_whole_blocks = buf_len >> BLOCK_SIZE_SHIFT;
        let len: u32;

        if num_whole_blocks << BLOCK_SIZE_SHIFT == buf_len {
            // Perfect fit!
            len = num_whole_blocks << BLOCK_SIZE_SHIFT;
        } else if buf_len <= BLOCK_SIZE {
            // This is a small allocation - only allocate the required number of whole-words
            len = match buf_len & 3 {
                0 => buf_len,
                1 => buf_len + 3,
                2 => buf_len + 2,
                3 => buf_len + 1,
                _ => unreachable!()
            };
        } else {
            // Since there is a partial block after this, account for it.
            len = (num_whole_blocks + 1) << 2;
        }

        // If the extra block space is too much, return null
        if len >= self.len {
            Ptr::<Chunk>::null()
        } else {
            unsafe {
                // Remaining space in this chunk, after accounting for a new Chunk and the len
                // of the requested buffer.
                let remaining_space = self.len - mem::size_of::<Chunk>() as u32 - len;

                // If the remaining space isn't a whole-block or more, use that space too.
                if remaining_space < BLOCK_SIZE {
                    // The current Chunk is good as is
                    self.remove_from_free_list();
                    Chunk::append_to_used(self.as_gba_ptr());
                    self.as_gba_ptr()
                } else {
                // Use the calculated amount of needed space, with some left over in new_chunk
                    // create new chunk that will hold remaining space
                    let mut new_chunk: Ptr<Chunk> = self.as_gba_ptr();
                    new_chunk.num += mem::size_of::<Chunk>() as u32 + len;

                    Chunk::initialize(&mut *new_chunk);

                    // new_chunk is a new node in the linked list - place it after the current node
                    new_chunk.next = self.next;
                    if !self.next.is_null() {
                        self.next.prev = new_chunk;
                    }
                    new_chunk.prev = Ptr::<Chunk>::from_mut_ref(self);

                    // New chunk will have any remaining space that is in this chunk
                    new_chunk.len = remaining_space;
                    self.len = len;

                    self.next = new_chunk;

                    self.remove_from_free_list();

                    let ptr = self.as_gba_ptr();
                    Chunk::append_to_used(ptr);
                    ptr
                }
            }
        }
    }
}

struct ChunkIterator {
    current: Option<Ptr<Chunk>>
}

impl<'a> iter::IntoIterator for &'a Chunk {
    type Item = Chunk;
    type IntoIter = ChunkIterator;

    fn into_iter(self) -> Self::IntoIter {
        let next = self.next;
        ChunkIterator { current: Some(next.prev) }
    }
}

impl iter::Iterator for ChunkIterator {
    type Item = Chunk;

    fn next(&mut self) -> Option<Chunk> {
        if self.current.is_none() {
            None
        } else {
            unsafe {
                let ch: Chunk = *(self.current.unwrap());
                if ch.next.is_null() {
                    self.current = None;
                } else {
                    self.current = Some(ch.next);
                }
                Some(ch)
            }
        }
    }
}

pub unsafe fn alloc_initialize() {
    let mut free_head: Ptr<Ptr<Chunk>> = Chunk::get_free_head();
    (*free_head).num = RAM_START;
    (**free_head).initialize();
    (**free_head).len = RAM_END - RAM_START - mem::size_of::<Chunk>() as u32;
}

pub unsafe fn alloc<T: Sized>(len: u32) -> Ptr<T> {
    let len = mem::size_of::<T>() as u32 * len;
    let head: Ptr<Ptr<Chunk>> = Chunk::get_free_head();
    let mut current = (*head).transmute::<Chunk>();
    while ! current.is_null() {
        let result: Ptr<Chunk> = current.try_alloc(len);
        if result.is_null() {
            current = (*current).next;
            continue
        }
        return (*result).get_data_ptr::<T>();
    }
    Ptr::<T>::null()
}

pub unsafe fn free<T: Sized>(ptr: &mut Ptr<T>) {
    if ptr.is_null() {
        return;
    } else {
        let mut chunk: Ptr<Chunk> = Ptr::<Chunk>::from_u32(ptr.num - (mem::size_of::<Chunk>() as u32));
        (*chunk).remove_from_used_list();
        Chunk::append_to_free(chunk);
        ptr.num = 0;
    }
}