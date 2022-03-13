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
//! # #[cfg(not(miri))]
//! const BASE: usize = 1024;
//! # #[cfg(miri)]
//! # const BASE: usize = 1;
//!
//! #[derive(DefaultBoxed)]
//! struct Foo {
//!     a: Bar,
//!     b: [Bar; 1024 * BASE],
//!     c: [u32; 1024 * BASE],
//! }
//!
//! struct Bar(u16);
//! impl Default for Bar {
//!     fn default() -> Bar {
//!         Bar(29)
//!     }
//! }
//!
//! let foo = Foo::default_boxed();
//! assert_eq!(foo.a.0, 29);
//! assert_eq!(foo.b[128 * BASE].0, 29);
//! assert_eq!(foo.c[256 * BASE], 0);
//!
//! let foo_arr = Foo::default_boxed_array::<16>();
//! assert_eq!(foo_arr[15].a.0, 29);
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
///
/// # Safety
///
/// Implementations must ensure that `default_in_place` initializes the value on the given pointer.
pub unsafe trait DefaultBoxed {
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

    /// Create a boxed array of the given size with default value of the type.
    ///
    /// ```
    /// use default_boxed::DefaultBoxed;
    /// let arr = u32::default_boxed_array::<32>();
    /// assert_eq!(arr, Box::new([0; 32]));
    /// ```
    fn default_boxed_array<const N: usize>() -> Box<[Self; N]>
    where
        Self: Sized,
    {
        let layout = Layout::new::<[Self; N]>();
        unsafe {
            if layout.size() == 0 {
                return Box::from_raw(ptr::NonNull::<[Self; N]>::dangling().as_ptr());
            }
            let raw = alloc_raw(layout) as *mut Self;
            if raw.is_null() {
                handle_alloc_error(layout)
            } else {
                for i in 0..N as isize {
                    Self::default_in_place(raw.offset(i));
                }
                Box::from_raw(raw as *mut [Self; N])
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

unsafe impl<T: Default> DefaultBoxed for T {
    unsafe fn default_in_place(ptr: *mut Self) {
        ptr::write(ptr, Default::default());
    }
}
