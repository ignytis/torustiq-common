#[repr(C)]
pub struct ByteBuffer {
    pub bytes: *mut u8,
    pub len: usize,
}

impl ByteBuffer {
    pub fn get_bytes_as_const_ptr(&self) -> *const i8 {
        unsafe { std::mem::transmute(self.bytes) }
    }
}

pub extern "C" fn generate_data() -> ByteBuffer {
    let mut buf = vec![0; 512].into_boxed_slice();
    let bytes = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);
    ByteBuffer { bytes, len }
}

pub extern "C" fn free_buf(buf: ByteBuffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.bytes, buf.len) };
    let s = s.as_mut_ptr();
    unsafe {
        let _ = Box::from_raw(s);
    }
}