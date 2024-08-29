# zips ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/cartercanedy/zips/test.yml?style=for-the-badge&label=tests) ![Crates.io Total Downloads](https://img.shields.io/crates/d/zips?style=for-the-badge&label=downloads)
just because I was annoyed that I couldn't use Option::zip with more than 2 arguments...

Zips provides 2 proc-macros that accept any number of either Option<T>'s or Result<T, E>'s as arguments and produces an Option<T\[, T...\]>.

## Installation
```sh
$> cargo add zips
```
---

## Usage with options:
```rust
use zips::{zip, zip_result};

fn main() -> () {
  let i: Option<i32> = Some(0);
  let j: Option<usize> = Some(1usize);
  let k: Option<()> = None;
 
  //  zipped_some: Option<(i32, usize)>
  let zipped_some = zip!(i, j);
  assert_eq!(zipped_some, Some((0, 1usize)));
 
  //  zipped_none: Option<(i32, usize, ())>
  let zipped_none = zip!(i, j, k);
  assert_eq!(zipped_none, None);


  let m: Result<i32, ()> = Ok(1);
  let n: Result<usize, ()> = Ok(0usize);
  let o: Result<(), &'static str> = Err("I'm an error!");
 
  //  zipped_ok: Option<(i32, usize)>
  let zipped_ok = zip_result!(m, n);
  assert_eq!(zipped_some, Some((1i32, 0usize)));
 
  //  zipped_err: Option<(i32, usize, ())>
  let zipped_err = zip_result!(m, n, o);
  assert_eq!(zipped_err, None);
}
```
