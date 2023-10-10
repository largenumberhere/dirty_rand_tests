use std::{alloc, mem};
use std::alloc::{dealloc, Layout};
use std::mem::size_of;
use std::ops::Deref;
use static_assertions::const_assert;

/// The most useless pseudo-random number generator you will ever see.
/// - It 'generates' numbers by allocating non-zerod memory and interpeting it as the required type.
/// - It is not secure - all randomness is grabbed from what may have been shared memory
/// - it is not very random -
///     1. Only when other parts of the program happen to allocate do we get an opportinity for viewing new sample data.
///     2. If no other allocation or deallocation happens, the same bytes will be recycled repeatedly for each call with no change.
///     3. We only have access to some parts of whatever happens to be mapped to our process' memory page. what is left there may very likely be sameish data from common opperations on allocated and then dealocated memory
/// - It is not scalable - long running program's DirtyRand values are even less random than a program that gets one random number than exits
/// - It wastes so many resources with constant allocations and deallocations for little valid reason.
/// It is however a fun way to play with the devil's unsafe rust. Let the undefined behaviour commence!
pub struct DirtyRand;
impl DirtyRand{
    pub fn make_u32() -> u32{
        let value: u32 = unsafe {
            /*Safety:
                - u32 is bitwise copyable
                - u32 is not and does not contain pointers or references
                - u32 is of byte size 4 on all platforms which is not 0
                - u32 can be validly represented for every number from 0x00000000 to 0xFFFFFFFF inclusive and such is all the possible values for 4 bytes
            */
            Self::make_unsafe()
        };

        value
    }

    pub fn make_u8() -> u8 {
        let value = unsafe {
            /*Safety:
                - u8 is bitwise copyable
                - u8 is not and does not contan pointers or references
                - u8 is of size 1 on all platofms which is not 0
                - u8 is an alias for a (unsigned) byte so it can be validly represented by one unit of itself
            */

            Self::make_unsafe::<u8>()
        };

        value
    }



    pub fn make_usize() -> usize{
        // Test at compile-time to make sure this architechture's usize isn't 0 just in case some weird platform exist like that
        const_assert!(size_of::<usize>() > 0); //The adress size of an achitechture cannot be 0...

        let val = unsafe{
            /*Safety:
                - any arch's usize is bitwise copyable because it is a primitive.
                - an integer cannot contain pointers or references
                - above, it was asserted that usize len must be greater than 0
                - It is assumed that all unsigned integers greater than 0 bytes can be by represented by any combination of n bytes
            */

            Self::make_unsafe::<usize>()
        };

        val


    }

    /// Create a random type of any kind. Panics if `Layout::new::<T>()` == 0.
    /// Please follow the safety points to prevent undefined bevahiour, none of the mentioned things are guarded against at any time in this method.
    /// You are responsible for ensuring no invalid states are reached.
    ///
    /// ## Safety:
    /// - T must be bit-wise copyable so it can be moved correctly with `core::ptr::read()`.
    /// - T must not be a refernce/pointer type or contain any of them, otherwise they may point to invalid or unaligned memory
    /// - T must be valid when represented by all combinations of `size_of<T>()` bytes, otherwise an invalid state may be constructed
    pub unsafe fn make_unsafe<T>() -> T {
        let len: usize = size_of::<T>();
        let layout = {
            Layout::new::<T>()
        };

        assert!(layout.size() > 0); // Panic before we request an alloc of size 0
        // grab some non-zeroed memory from the os
        let garbage_memory = unsafe{

            /* Safety:
                - *this method's user must* ensure T is never of size 0, as stated in method documentation
                - the layout is not yet initialized because we only initialize it once here
            */
            alloc::alloc(layout)

        };
        let garbage_ref = garbage_memory as *mut T;

        // 'Move out' the bytes by copying here
        let value = unsafe{
            /* Safety:
             - T is valid for reads because we just allocated it as readable/writable memory
             - T is properly aligned because our given `layout` was created with `Layout::new` which checks for valid alignment
             This *method user must* ensure that
             - T must be correclty initializable from raw bytes as clatified in method docs
            */
            garbage_ref.read()
        };

        // Memory leak bad
        unsafe{
            /*Safety:
                - We know that garbage memory was allocated as seen in the method earlier.
                - We take care to dealloc the memory with the same one as it was allocated with.
            */
            dealloc(garbage_memory, layout)

        };

        return value;
    }

}
