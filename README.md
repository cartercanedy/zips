## zips
just because I was annoyed that I couldn't use Option::zip with more than 2 arguments...


## Installation
```sh
$> cargo add zips
```
---

## Usage:
```rust
use zips::zip;

fn main() -> () {
    let zipped = zip!(Some(0), Some(1));
    assert_eq!(zipped, Some((0, 1)));

    let zipped_none: Option<((i32, i32), i32)> = zip!(zipped, None);
    assert_eq!(zipped_none, None)
}
```
