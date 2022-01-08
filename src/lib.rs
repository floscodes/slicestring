//! slicestring is a crate for slicing Strings.
//! It provides the [`slice()`] method for [`String`] and [`str`].
//! It takes the index-range as an argument.
//! It slices the [`String`] or [`&str`] and returns a the sliced one as a [`String`].
//! 
//! # Example:
//! 
//! ```
//! use slicestring::Slice;
//! 
//! let mut s = "hello world!";
//! s = s.slice(..5);
//! assert_eq!("hello", s);
//! ```
//! 
//! It also works with emoticons since the [`slice()`] method takes into account characters.
//! 
//! ```
//! let mut s = String::from("hello 😃");
//! s = s.slice(5..);
//! assert_eq!("😃", s);
//! ```
//! 
//! [`slice()`]: trait.Slice.html#tymethod.slice


/// Provides the [`slice()`] method.
/// [`slice()`]: trait.Slice.html#tymethod.slice
pub trait Slice {
    fn slice(&self, r: impl core::ops::RangeBounds<usize>) -> String;
}

impl Slice for String {

/// The [`slice()`] method is provided for [`std::string::String`] and takes the index-range as an argument.
/// It slices the [`String`] returns a new one.
/// 
/// Example:
/// ```
/// let mut s = String::from("hello world!");
/// s = s.slice(..5);
/// assert_eq!("hello", s);
/// ```
fn slice(&self, r: impl core::ops::RangeBounds<usize>) -> String {

    use std::ops::Bound::*;

    let x = match r.start_bound() {
        Included(&i) => i,
        Excluded(&i) => i,
        Unbounded => 0,
    };

    let y = match r.end_bound() {
        Included(&i) => i,
        Excluded(&i) => i,
        Unbounded => self.len()
    };

    let mut new = String::new();

    for (i, c) in self.char_indices() {
        if i >= x && i < y {
            new.push(c);
        }
    }

    new

}
   
}

//implementing the Slice-trait for &str

impl Slice for str {
/// The [`slice()`] method is provided for [`&str`] and takes the index-range as an argument.
/// It slices the [`&str`] and returns a the sliced one as a [`String`].
/// 
/// Example:
/// ```
/// let mut s = String::from("hello world!");
/// s = s.slice(..5);
/// assert_eq!("hello", s);
/// ```
fn slice(&self, r: impl core::ops::RangeBounds<usize>) -> String {

    use std::ops::Bound::*;

    let x = match r.start_bound() {
        Included(&i) => i,
        Excluded(&i) => i,
        Unbounded => 0,
    };

    let y = match r.end_bound() {
        Included(&i) => i,
        Excluded(&i) => i,
        Unbounded => self.len()
    };

    let mut new = String::new();

    for (i, c) in self.char_indices() {
        if i >= x && i < y {
            new.push(c);
        }
    }

    new

}
}

#[test]
fn test() {
    let s = "test";

    println!("{}", s.slice(..2));
}