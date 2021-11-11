use std::sync::{Arc, LockResult, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A struct that has a single writer for data with many readers.
pub struct Writer<T>(Arc<RwLock<T>>);

impl<T> Writer<T> {
    /// Creates a new reader.
    pub fn make_reader(&self) -> Reader<T> {
        Reader(Arc::clone(&self.0))
    }

    /// Creates a new single writer for the data.
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }

    /// Attempts to read the given data
    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        self.0.read()
    }

    /// Attempts to write the data.
    pub fn write(
        &mut self,
    ) -> Result<RwLockWriteGuard<'_, T>, PoisonError<RwLockWriteGuard<'_, T>>> {
        self.0.write()
    }
}

impl<T> Clone for Writer<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// A reader for some data
pub struct Reader<T>(Arc<RwLock<T>>);

impl<T> Reader<T> {
    /// Attempts to read the given data
    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        self.0.read()
    }
}

impl<T> Clone for Reader<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
