# String Pattern Matching

## Example

```rust
let x = "2a343bb8 c9";
assert_eq!(Some(("2", "343", "8", "9")), match_str!(x, {} "a" {} "bb" {} " c" {}));
assert_eq!(None, match_str!(x, {} "a" {} "d" {})); // x doesn't contain "d"
assert_eq!(Some("a343bb8 c"), match_str!(x, "2" {} "9"));
assert_eq!(None, match_str!(x, "a" {} "c")); // x doesn't start with "a" and end with "c"
```