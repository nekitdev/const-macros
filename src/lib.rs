//! Various macros for const contexts.
//!
//! # Examples
//!
//! Below is an example that utilizes most of the macros provided by this crate.
//!
//! ```
//! use const_macros::{const_assert, const_early, const_ok, const_try};
//!
//! use thiserror::Error;
//!
//! #[derive(Error, Debug)]
//! #[error("unexpected length: `{value}`")]
//! pub struct Error {
//!     pub value: usize,
//! }
//!
//! impl Error {
//!     pub const fn new(value: usize) -> Self {
//!         Self { value }
//!     }
//! }
//!
//! pub const MIN: usize = 32;
//! pub const MAX: usize = 96;
//!
//! const_assert!(MIN <= MAX);
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
//! pub struct Length {
//!     value: usize,
//! }
//!
//! impl Length {
//!     pub const fn new(value: usize) -> Result<Self, Error> {
//!         const_try!(Self::check(value));
//!
//!         Ok(unsafe { Self::new_unchecked(value) })
//!     }
//!
//!     pub const fn new_ok(value: usize) -> Option<Self> {
//!         const_ok!(Self::new(value))
//!     }
//!
//!     pub const fn check(value: usize) -> Result<(), Error> {
//!         const_early!(value < MIN || value > MAX => Error::new(value));
//!
//!         Ok(())
//!     }
//!
//!     pub const unsafe fn new_unchecked(value: usize) -> Self {
//!         Self { value }
//!     }
//!
//!     pub const MIN: Self = Self::new_ok(MIN).unwrap();
//!     pub const MAX: Self = Self::new_ok(MAX).unwrap();
//! }
//! ```

#![no_std]
#![deny(missing_docs)]

#[allow(unused_imports)]
#[doc(hidden)]
pub mod import {
    pub use core::{option::Option, result::Result};
}

/// Similar to the `?` operator used on [`Result`] but for `const` contexts.
///
/// Note that no conversions are performed, as it is impossible in `const` contexts.
#[macro_export]
macro_rules! const_try {
    ($result: expr) => {
        match $result {
            $crate::import::Result::Ok(value) => value,
            $crate::import::Result::Err(error) => return $crate::import::Result::Err(error),
        }
    };
}

/// Equivalent to the `?` operator used on [`Option`] but for `const` contexts.
#[macro_export]
macro_rules! const_none {
    ($option: expr) => {
        match $option {
            $crate::import::Option::Some(value) => value,
            $crate::import::Option::None => return $crate::import::Option::None,
        }
    };
}

/// Same as [`Option::map`] but for `const` contexts.
#[macro_export]
macro_rules! const_map {
    ($option: expr => $function: expr) => {
        match $option {
            $crate::import::Option::Some(value) => $crate::import::Option::Some($function(value)),
            $crate::import::Option::None => $crate::import::Option::None,
        }
    };
}

/// Same as [`Result::ok`] but for `const` contexts.
#[macro_export]
macro_rules! const_ok {
    ($result: expr) => {
        match $result {
            $crate::import::Result::Ok(value) => $crate::import::Option::Some(value),
            $crate::import::Result::Err(_) => $crate::import::Option::None,
        }
    };
}

/// Same as [`Result::err`] but for `const` contexts.
#[macro_export]
macro_rules! const_err {
    ($result: expr) => {
        match $result {
            $crate::import::Result::Ok(_) => $crate::import::Option::None,
            $crate::import::Result::Err(error) => $crate::import::Option::Some(error),
        }
    };
}

/// Same as [`Result::map`] but for `const` contexts.
#[macro_export]
macro_rules! const_map_ok {
    ($result: expr => $function: expr) => {
        match $result {
            $crate::import::Result::Ok(value) => $crate::import::Result::Ok($function(value)),
            $crate::import::Result::Err(error) => $crate::import::Result::Err(error),
        }
    };
}

/// Same as [`Result::map_err`] but for `const` contexts.
#[macro_export]
macro_rules! const_map_err {
    ($result: expr => $function: expr) => {
        match $result {
            $crate::import::Result::Ok(value) => $crate::import::Result::Ok(value),
            $crate::import::Result::Err(error) => $crate::import::Result::Err($function(error)),
        }
    };
}

/// Returns early with the provided error if the condition is true.
#[macro_export]
macro_rules! const_early {
    ($condition: expr => $error: expr) => {
        if $condition {
            return $crate::import::Result::Err($error);
        }
    };
}

/// Returns early with [`None`] if the condition is true.
#[macro_export]
macro_rules! const_quick {
    ($condition: expr) => {
        if $condition {
            return $crate::import::Option::None;
        }
    };
}

/// Similar to [`assert!`] but for `const` contexts.
#[macro_export]
macro_rules! const_assert {
    ($condition: expr) => {
        const _: () = assert!($condition);
    };
}

/// Similar to [`assert_eq!`] but for `const` contexts.
#[macro_export]
macro_rules! const_assert_eq {
    ($left: expr, $right: expr $(,)?) => {
        $crate::const_assert!($left == $right);
    };
}

/// Similar to [`assert_ne!`] but for `const` contexts.
#[macro_export]
macro_rules! const_assert_ne {
    ($left: expr, $right: expr $(,)?) => {
        $crate::const_assert!($left != $right);
    };
}
