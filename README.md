# arossbell Rust Vec Extensions [![](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/arossbell/arossbell-vec-extensions/blob/master/LICENSE.md)

This library contains some Rust Vec-type extensions that make some Vec-type-related functions fit better in my code.

## Inclusion Example
Make your Cargo.toml do something like this:
```toml
[dependencies]
arossbell-vec-extensions = {git = "https://github.com/arossbell/arossbell-vec-extensions"}
```
... then ...
```rust
extern crate arossbell_vec_extensions;

use arossbell_vec_extensions::*;
```
## Functions
### Prepend a value to a vector
```rust
assert_eq!(
    vec![2,3,4].prepend_value(&1),
    vec![1,2,3,4],
);
```

### Postpend a value to a vector
```rust
assert_eq!(
    vec![1,2,3].postpend_value(&4),
    vec![1,2,3,4],
);
```
