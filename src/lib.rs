//! slicestring is a crate for slicing Strings.
//! It provides the [`slice()`] method for [`std::string::String`].
//! It takes two arguments: The start-index and the end-index and returns a [`String`].
//! 
//! #Example:
//! 
//! ```
//! use slicestring::Slice;
//! 
//! let mut s = String::from("hello world!");
//! s = s.slice(0, 5);
//! assert_eq!("hello", s);
//! ```
//! 
//! It also works with emoticons since the [`slice()`] method takes into account characters.
//! 
//! ```
//! let mut s = String::from("hello 😃");
//! s = s.slice(5, s.len());
//! assert_eq!("😃", s);
//! ```

/// Provides the [`slice()`] method.
pub trait Slice {

    /// The [`slice()`] method is provided for [`std::string::String`] and takes two arguments:
    /// The start-index and the end-index. It returns a sliced [`String`].
    fn slice(&self, x: usize, y: usize) -> Self;
}

impl Slice for String {

/// The [`slice()`] method is provided for [`std::string::String`] and takes two arguments:
/// The start-index and the end-index. It returns a sliced [`String`].
/// 
/// Example:
/// ```
/// let mut s = String::from("hello world!");
/// s = s.slice(0, 5);
/// assert_eq!("hello", s);
/// ```
fn slice(&self, x: usize, y: usize) -> String {

    let mut new = String::new();

    for (i, c) in self.char_indices() {
        if i >= x && i < y {
            new.push(c);
        }
    }

    new

}
   
}