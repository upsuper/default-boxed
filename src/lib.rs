pub use default_boxed_derive::DefaultBoxed;

/// Helper trait to create a boxed instance of the given type with a default value for each field.
///
/// This trait can be derived for structs with named fields.
///
/// To derive this trait, each field needs to also implement this trait, but all types which
/// implements `Default` implements this trait via the blanket `impl` already.
///
/// In addition, if a field is an array, only the item type needs to implement this trait, and each
/// item would be initialized separately.
pub trait DefaultBoxed {
    /// Create a boxed instance with default value for each field.
    fn default_boxed() -> Box<Self> where Self: Sized {
        let mut v = Vec::with_capacity(1);
        let raw = v.as_mut_ptr();
        std::mem::forget(v);
        unsafe {
            Self::default_in_place(raw);
            Box::from_raw(raw)
        }
    }

    /// Fill the given memory location with default value.
    ///
    /// # Safety
    ///
    /// For callers, behavior is undefined if `ptr` is not valid for writes, or it is not properly
    /// aligned.
    ///
    /// For impls, behavior is undefined if this method reads from `ptr`.
    unsafe fn default_in_place(ptr: *mut Self);
}

impl<T: Default> DefaultBoxed for T {
    unsafe fn default_in_place(ptr: *mut Self) {
        std::ptr::write(ptr, Default::default());
    }
}
