use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout}, mem, ptr::{copy_nonoverlapping, read, write, write_bytes}
};

use super::memory_handler::MemoryHandler;

pub struct ReByte<T> {
    pub(crate) initialized: bool,
    pub(crate) size: Option<usize>,
    pub(crate) handler: Option<*const u8>,
    pub(crate) layout: Option<Layout>,
    _phantom: Option<T>
}

impl<'a, T> MemoryHandler<'a, T> for ReByte<T> {
    fn get_value_alloc(&self) -> T {
        if !self.initialized {
            panic!("Rebyte is not initialized.");
        }

        unsafe {
            return read(self.handler.unwrap() as *const T);
        }
    }

    fn get_value(&self) -> &T {
        if !self.initialized {
            panic!("Rebyte is not initialized.");
        }

        unsafe {
            let raw_ptr = self.handler.unwrap() as *const T;
            // Convert the raw pointer to a reference
            &*raw_ptr
        }
    }


    fn set_value(&self, value: T) {
        if !self.initialized {
            panic!("Rebyte is not initialized.");
        }

        unsafe {
            write(self.handler.unwrap() as *mut T, value);
        }
    }

    fn set_value_bytes(&self, value: &[u8]) {
        if !self.initialized {
            panic!("Rebyte is not initialized.");
        }
        
        unsafe { 
            copy_nonoverlapping(value.as_ptr(), self.handler.unwrap() as *mut u8, value.len()) 
        }
    }

    fn get_size(&self) -> usize {
        if !self.initialized {
            panic!("Rebyte is not initialized.");
        }
        
        self.size.unwrap()
    }

    fn get_handler(&self) -> *const u8 {
        if !self.initialized {
            panic!("Rebyte is not initialized.");
        }
        
        self.handler.unwrap()
    }

    fn allocate(&mut self) {
        if self.initialized {
            panic!("Rebyte is already allocated. Use clear if you need to clear the memory block");
        }

        unsafe {
            let len = std::mem::size_of::<T>();
            let layout = Layout::array::<T>(len).unwrap();
        
            let ptr = alloc(layout);
    
            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            self.initialized = true;
            self.handler = Some(ptr);
            self.size = Some(len);
            self.layout = Some(layout);
        }
    }

    fn clear(&self) {
        if !self.initialized {
            panic!("Rebyte is not initialized.");
        }
        
        unsafe {
            write_bytes(self.handler.unwrap() as *mut u8, 0, self.size.unwrap());
        }
    }
    
    fn get_bytes(&self) -> &'a [u8] {
        unsafe {
            return std::slice::from_raw_parts(self.handler.unwrap(), self.size.unwrap())
        }
    }
}

impl<T> Drop for ReByte<T> {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                dealloc(self.handler.unwrap() as *mut u8, self.layout.unwrap());
            }
        }
    }
}

impl<T> Default for ReByte<T>  {
    fn default() -> Self {
        Self { 
            initialized: false, 
            size: None, 
            handler: None, 
            layout: None, 
            _phantom: None
        }
    }
}

/// Gets the bytes out of a struct
pub fn get_struct_bytes<'a, T>(value: T) -> &'a [u8] {
    unsafe {
        let bytes: &[u8] = std::slice::from_raw_parts(
            &value as *const _ as *const u8,
            mem::size_of::<T>(),
        );
        
        drop(value);

        return bytes;
    }
}
