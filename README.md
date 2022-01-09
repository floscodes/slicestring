# slicestring

slicestring is a crate for slicing Strings.
It provides the `slice()` method for `String` and `&str`.
It takes the index-range as an argument, whereby also a negative value can be passed for the second index.
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

You can also use a negative value for the second index.

```
use slicestring::Slice;

let mut s = String::from("hello");
s = s.slice(..-3);
assert_eq!("he", s);
```

or

```
use slicestring::Slice;

let mut s = String::from("hello");
s = s.slice(1..-1);
assert_eq!("ell", s);
```