# slicestring

slicestring is a crate for slicing Strings.
It provides the `slice` method for `std::string::String`.
It takes two arguments: The start-index and the end-index and returns a `String`.

## Examples:

```
use slicestring::Slice;

let mut s = String::from("hello world!");
s = s.slice(0, 5);
assert_eq!("hello", s);
```

It also works with emoticons since the `slice` method takes into account characters.

```
let mut s = String::from("hello 😃");
s = s.slice(5, s.len());
assert_eq!("😃", s);
```