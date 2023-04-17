// Buffers utils

use std::mem;

/// Align a number to a multiple of four.
pub fn align_to_multiple_of_four(n: &mut u32) {
    *n = (*n + 3) & !3;
}

/// Pads a vector of bytes to a multiple of four.
pub fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}
