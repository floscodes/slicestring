# slicestring

slicestring is a crate for slicing Strings.
It provides the `slice()` method for `String` and `&str`.
It takes a `std::ops::Range` as an argument.
It slices the `String` or `&str` and returns the sliced one as a `String`.

## Examples:

```
use slicestring::Slice;

let mut s = "hello world!";
s = s.slice(..5);
assert_eq!("hello", s);
```

It also works with emoticons since the `slice` method takes into account characters.

```
use slicestring::Slice;

let mut s = String::from("hello 😃");
s = s.slice(5..);
assert_eq!("😃", s);
```
