# slicestring

slicestring is a crate for slicing Strings.
It provides the `slice` method for `std::string::String`.
It provides the [`slice()`] method for [`std::string::String`] and takes the index-range as an argument.
It slices the [`String`] returns a new one.

## Examples:

```
use slicestring::Slice;

let mut s = String::from("hello world!");
s = s.slice(..5);
assert_eq!("hello", s);
```

It also works with emoticons since the `slice` method takes into account characters.

```
let mut s = String::from("hello 😃");
s = s.slice(5..);
assert_eq!("😃", s);
```
