//! # My crate
//!
//! `my_crate` is a collection of utilities to omake performing certain calculations more
//! convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```rust
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

