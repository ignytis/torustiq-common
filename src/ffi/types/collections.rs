use crate::ffi::types::std_types;

#[repr(C)]
pub struct Array<T> {
    pub data: *mut T,
    pub len: std_types::Uint,
}

impl<T> Array<T> {
    pub fn new_of_len(len: usize) -> Array<T> {
        let mut data: Vec<T> = Vec::with_capacity(len);
        Array {
            data: unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr(), std::mem::size_of::<T>() * len).as_mut_ptr() },
            len: len as std_types::Uint,
        }
    }

    pub fn from_vec(vector: Vec<T>) -> Array<T> {
        let len = vector.len() as std_types::Uint;
        let mut boxed_slice: Box<[T]> = vector.into_boxed_slice();
        let arr: Array<T> = Array {
            data: boxed_slice.as_mut_ptr(),
            len,
        };
        std::mem::forget(boxed_slice);
        arr
    }
}

pub fn free_array<T>(arr: Array<T>) {
    let s = unsafe { std::slice::from_raw_parts_mut(arr.data, arr.len as usize) };
    let s = s.as_mut_ptr();
    unsafe {
        let _ = Box::from_raw(s);
    }
}