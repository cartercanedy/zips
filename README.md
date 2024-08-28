## zips
just because I was annoyed that I couldn't use Option::zip with more than 2 arguments...

[![testing](https://github.com/cartercanedy/zips/actions/workflows/rust.yml/badge.svg)](https://github.com/cartercanedy/zips/actions/workflows/rust.yml)

## Installation
```sh
$> cargo add zips
```
---

## Usage:
```rust
use zips::{zip, zip_result};

fn main() -> () {
  let i: Option<i32> = Some(0);
  let j: Option<usize> = Some(1usize);
  let k: Option<usize> = None;
 
  //  zipped_some: Option<(i32, usize)>
  let zipped_some = zip!(i, j);
  assert_eq!(zipped_some, Some((0, 1usize)));
 
  //  zipped_none: Option<(i32, usize, usize)>
  let zipped_none = zip!(i, j, j);
  assert_eq!(zipped_none, None);


  let m: Result<i32, String> = Ok(1);
  let n: Result<usize, String> = Ok(0usize);
  let o: Result<usize, String> = Err("I'm an error!");
 
  //  zipped_ok: Option<(i32, usize)>
  let zipped_ok = zip_result!(m, n);
  assert_eq!(zipped_some, Some((1, 0usize)));
 
  //  zipped_err: Option<(i32, usize, usize)>
  let zipped_err = zip_result!(m, n, o);
  assert_eq!(zipped_err, None);
}
```
