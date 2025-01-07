use crate::ffi::utils::strings::bytes_to_string_safe;

#[repr(C)]
#[derive(Copy)]
pub struct ByteBuffer {
    pub bytes: *mut u8,
    pub len: usize,
}

impl ByteBuffer {
    pub fn get_bytes_as_const_ptr<T>(&self) -> *const T {
        unsafe { std::mem::transmute(self.bytes) }
    }

    pub fn to_string(&self) -> String {
        bytes_to_string_safe(self.bytes, self.len)
    }

    pub fn to_byte_vec(&self) -> Vec<u8> {
        let mut dst: Vec<u8> = Vec::with_capacity(self.len);
        unsafe {
            std::ptr::copy(self.bytes, dst.as_mut_ptr(), self.len);
             // NB: set_len is needed here; setting the capacity is not enough
            dst.set_len(self.len);
        };
        dst
    }

    pub fn free_contents(&mut self) {
        let _ = unsafe { Box::from_raw(self.bytes) };
        self.len = 0;
    }
}

impl Clone for ByteBuffer {
    fn clone(&self) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.len);
        unsafe { std::ptr::copy(self.bytes, bytes.as_mut_ptr(), self.len) };
        let ptr = bytes.as_mut_ptr();
        std::mem::forget(bytes);
        ByteBuffer {
            len: self.len,
            bytes: ptr,
        }
    }
}

impl From<Vec<u8>> for ByteBuffer {
    fn from(value: Vec<u8>) -> Self {
        let mut dst: Vec<u8> = value.clone();
        let bytes = dst.as_mut_ptr();
        std::mem::forget(dst);

        ByteBuffer {
            bytes,
            len: value.len(),
        }
    }
}

impl From<String> for ByteBuffer {
    fn from(input: String) -> Self {
        let mut dst: Vec<u8> = Vec::with_capacity(input.len());
        unsafe { std::ptr::copy(input.as_ptr(), dst.as_mut_ptr(), input.len()) };
        let bytes = dst.as_mut_ptr();
        std::mem::forget(dst);

        ByteBuffer {
            bytes,
            len: input.len(),
        }
    }
}

pub extern "C" fn free_buf(buf: ByteBuffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.bytes, buf.len) };
    let s = s.as_mut_ptr();
    unsafe {
        let _ = Box::from_raw(s);
    }
}