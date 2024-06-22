#[repr(C)]
pub struct ByteBuffer {
    pub bytes: *mut u8,
    pub len: usize,
}

impl ByteBuffer {
    pub fn get_bytes_as_const_ptr<T>(&self) -> *const T {
        unsafe { std::mem::transmute(self.bytes) }
    }

    pub fn from_string(input: String) -> Self {
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