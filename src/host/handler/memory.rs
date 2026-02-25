use std::sync::{LazyLock, RwLock, RwLockWriteGuard};

static BUFFER: RwLock<LazyLock<Buffer>> = RwLock::new(LazyLock::new(Buffer::new));
const SIZE: usize = 2048;

pub(crate) struct Buffer {
    data: [u8; SIZE],
}
impl Buffer {
    fn new() -> Buffer {
        Self { data: [0u8; SIZE] }
    }
    #[inline]
    pub(crate) fn len(&self) -> i32 {
        self.data.len() as i32
    }
    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub(crate) fn as_subslice(&self, size: i32) -> &[u8] {
        &self.data[0..size as usize]
    }
    /// returns a copy of the contents as an owned type
    pub(crate) fn to_boxed_slice(&self, size: i32) -> Box<[u8]> {
        Box::from(self.as_subslice(size))
    }
    #[cfg(test)]
    pub(super) fn from_slice(data: &[u8]) -> Buffer {
        let mut buffer = [0; SIZE];
        buffer[..data.len()].clone_from_slice(data);
        Self { data: buffer }
    }
}

pub(crate) fn buffer() -> RwLockWriteGuard<'static, LazyLock<Buffer>> {
    match BUFFER.write() {
        Ok(guard) => guard,
        Err(poisoned) => {
            BUFFER.clear_poison();
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};
    use std::thread;

    #[test]
    fn test_as_slice() {
        let c = b"test";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len() as i32);
        assert_eq!(c, r);
    }
    #[test]
    fn test_as_slice_empty() {
        let c = b"";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len() as i32);
        assert!(r.is_empty());
    }

    #[test]
    fn test_lock_recovers_from_poison_and_clears_it() {
        // Use a local lock wrapped in Arc to avoid affecting global state
        let lock = Arc::new(RwLock::new(42u32));

        // Poison the lock by panicking while holding it
        let lock_clone = Arc::clone(&lock);
        let result = thread::spawn(move || {
            let _guard = lock_clone.write().unwrap();
            panic!("intentional panic to poison lock");
        })
        .join();

        // Verify the thread panicked
        assert!(result.is_err());

        // Verify the lock is poisoned
        assert!(lock.is_poisoned());

        // Recover from the poisoned lock and clear the poison flag
        {
            let guard = match lock.write() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    lock.clear_poison();
                    poisoned.into_inner()
                }
            };
            assert_eq!(*guard, 42);
        }

        // Verify the lock is no longer poisoned
        assert!(!lock.is_poisoned());

        // Verify we can now acquire the lock normally
        let guard = lock.write().expect("lock should not be poisoned");
        assert_eq!(*guard, 42);
    }
}
