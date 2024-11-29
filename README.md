# String Pattern Matching

## Features

- Fast
- Simple

## Example

```rust
let x = "2a343bb8 c9";
assert_eq!(Some(("2", "343", "8", "9")), match_str!(x, {} "a" {} "bb" {} " c" {}));
assert_eq!(None, match_str!(x, {} "a" {} "d" {})); // x doesn't contain "d"
assert_eq!(Some("a343bb8 c"), match_str!(x, "2" {} "9"));
assert_eq!(None, match_str!(x, "a" {} "c")); // x doesn't start with "a" and end with "c"
```

---

```rust
let x = match_str!("foo bar baz fuzz", {} "bar" {} "fuzz");
assert_eq!(Some(("foo ", " baz ")), x);
```

macro expansion:

```rust
let x = {
    let s = strpatmatch::first_match_start("foo bar baz fuzz", "bar");
    if let Some(s) = s {
        if let Some(m) = {
            if (&"foo bar baz fuzz"[s + "bar".len()..]).ends_with("fuzz") {
                Some((
                    &(&"foo bar baz fuzz"[s
                        + "bar"
                        .len() - "fuzz".len())],
                ))
            } else {
                None
            }
        } {
            Some(strpatmatch::tuples::concat((&"foo bar baz fuzz"[0..s],), m))
        } else {
            None
        }
    } else {
        None
    }
};
// assert_eq! ... 
```