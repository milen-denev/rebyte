pub trait MemoryHandler<'a, T> {
    /// Retrieves a copy the value from self
    fn get_value_alloc(&self) -> T;

    /// Retrieves a reference of the value from self
    fn get_value(&self) -> &T;

    /// Sets the value of the memory allocation from self
    fn set_value(&self, value: T);

    /// Sets the value of the memory allocation using bytes from self
    fn set_value_bytes(&self, value: &[u8]);

    /// Gets the size of generic T
    fn get_size(&self) -> usize;

    /// Get the raw pointer of the memory block
    fn get_handler(&self) -> *const u8;

    /// Allocates the a memory block on the heap memory
    fn allocate(&mut self); 

    /// Clears the memory block
    fn clear(&self);

    fn get_bytes(&self) -> &'a [u8];
}