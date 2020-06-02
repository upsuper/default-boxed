#![cfg_attr(not(doctest), no_std)]

//! Helper trait to create instances of large structs with default value on heap directly
//! without going through stack.
//!
//! Similar to the unstable `box` syntax,
//! it semantically doesn't require creating the whole struct on stack then moving to heap,
//! and thus unlike [`copyless`][copyless] or [`boxext`][boxext],
//! it doesn't rely on optimization to eliminate building the struct on stack,
//! which may still face stack overflow on debug build when creating large struct.
//!
//! [copyless]: https://crates.io/crates/copyless
//! [boxext]: https://crates.io/crates/boxext
//!
//! ## Example
//!
//! ```
//! use default_boxed::DefaultBoxed;
//!
//! #[derive(DefaultBoxed)]
//! struct Foo {
//!     a: Bar,
//!     b: [Bar; 1024 * 1024],
//!     c: [u32; 1024 * 1024],
//! }
//!
//! struct Bar(u16);
//! impl Default for Bar {
//!     fn default() -> Bar {
//!         Bar(29)
//!     }
//! }
//!
//! #[test]
//! fn test_basic() {
//!     let foo = Foo::default_boxed();
//!     assert_eq!(foo.a.0, 29);
//!     assert_eq!(foo.b[128 * 1024].0, 29);
//!     assert_eq!(foo.c[256 * 1024], 0);
//! }
//! ```

extern crate alloc;

use alloc::alloc::{alloc as alloc_raw, handle_alloc_error, Layout};
use alloc::boxed::Box;
use core::ptr;

pub use default_boxed_derive::DefaultBoxed;

/// Helper trait to create a boxed instance of the given type with a default value for each field.
///
/// This trait can be derived for structs.
///
/// To derive this trait, each field needs to also implement this trait, but all types which
/// implements `Default` implements this trait via the blanket `impl` already.
///
/// In addition, if a field is an array, only the item type needs to implement this trait, and each
/// item would be initialized separately.
pub trait DefaultBoxed {
    /// Create a boxed instance with default value for each field.
    fn default_boxed() -> Box<Self>
    where
        Self: Sized,
    {
        let layout = Layout::new::<Self>();
        unsafe {
            if layout.size() == 0 {
                return Box::from_raw(ptr::NonNull::<Self>::dangling().as_ptr());
            }
            let raw = alloc_raw(layout) as *mut Self;
            if raw.is_null() {
                handle_alloc_error(layout)
            } else {
                Self::default_in_place(raw);
                Box::from_raw(raw)
            }
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
        ptr::write(ptr, Default::default());
    }
}
