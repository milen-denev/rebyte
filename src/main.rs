use std::sync::atomic::{AtomicBool, Ordering};

use bytes::BytesMut;


fn main() {
    println!("Hello, world!");
}

pub struct ReByte { 
    pub(crate) chunks: Vec<Chunk>,
    pub(crate) bytes: BytesMut,
    pub(crate) last_address: *const u8
}

pub(crate) struct Chunk {
    pub(crate) address: usize,
    pub(crate) inner_bytes: BytesMut,
    pub(crate) in_use: AtomicBool 
}

impl ReByte {
    pub fn new(with_size: usize) -> ReByte {
        ReByte {
            chunks: vec![],
            bytes: BytesMut::with_capacity(with_size)
        }
    }

    fn add_chunk(&mut self, capacity: usize) {
        unsafe {
            let pointer = self.bytes.as_ptr();
            let current_size = self.bytes.len();
            
            let slice = unsafe { std::slice::from_raw_parts(some_pointer, count_of_items) };

            let chunk_bytes = BytesMut::from(pointer);
    
            let chunk = Chunk {
                address: chunk_bytes.as_ptr(),
                inner_bytes: chunk_bytes,
                in_use: AtomicBool::new(false)
            };
    
            self.chunks.push(chunk);
        }
    }

    fn resize(&mut self, additional_capacity: usize) {
        let bts: &[u8] = &vec![0; additional_capacity];
        self.bytes.extend_from_slice(bts);
    }

    fn get_size(&self) -> usize {
        self.bytes.len()
    }

    fn return_chunk(&self, chunk: Chunk) {

    }

    pub fn get_chunk<'a>(&'a mut self, capacity: usize) {//-> &'a [u8] {
        for chunk in self.chunks.iter().filter(|x| !x.in_use.load(Ordering::Relaxed)) {
            
        }


    }

}