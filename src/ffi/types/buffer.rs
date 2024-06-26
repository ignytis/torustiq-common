#[repr(C)]
pub struct ByteBuffer {
    pub bytes: *mut u8,
    pub len: usize,
}

impl ByteBuffer {
    pub fn get_bytes_as_const_ptr<T>(&self) -> *const T {
        unsafe { std::mem::transmute(self.bytes) }
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

// TODO: check for memory leaks and start using this function
pub extern "C" fn free_buf(buf: ByteBuffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.bytes, buf.len) };
    let s = s.as_mut_ptr();
    unsafe {
        let _ = Box::from_raw(s);
    }
}