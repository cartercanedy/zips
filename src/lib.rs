// Copyright (c) 2024 Carter Canedy <cartercanedy42@gmail.com>

#![feature(macro_metavar_expr)]

/// Zips multiple instances of Option<T> into a single instance of Option<(T [, T...])>. Will
/// either be Some if all arguments are some, or None if one of the instances is None.
///
/// ## Usage:
/// ```
/// let zipped_some = zip!(Some(1), Some(2));
/// assert_eq!(zipped_some, Some((1, 2)));
///
/// let zipped_none = zip!(Some(1), None);
/// assert_eq!(zipped_none, None);
/// ```
#[macro_export]
macro_rules! zip {
  ($($args:expr),+) => ({
    let mut ok = true;
    $(
      let arg${index()} = $args;
      if arg${index()}.is_none() {
        ok = false;
      }
     )+

    if ok {
      Some(($(arg${index()}.unwrap(),)+))
    } else {
      None
    }
  });
}

/// Zips an arbitrary number of `Result<T>` into a single instance of `Option<(T [, T...])>`. If
/// any arguments are `Err`, `None` is returned. Otherwise, `Some` is returned.
///
/// ## Usage:
/// ```
/// let i: Result<i32, String> = Ok(1);
/// let j: Result<usize, String> = Ok(0usize);
/// let k: Result<usize, String> = Err("I'm an error");
///
/// // zipped_ok: Option<(i32, usize)>
/// let zipped_ok = zip_result!(i, j);
/// assert_eq!(zipped_ok, Some(1, 0usize));
///
/// // zipped_err: Option<(i32, usize, usize)> 
/// let zipped_err = zip_result!(i, j, k);
/// assert_eq!(zipped_err, None);
/// ```
#[macro_export]
macro_rules! zip_result {
  ($($args:expr),+) => ({
    let mut ok = true;
    $(
      if $args.is_err() {
        ok = false;
      }
    )+

    if ok {
      Some(($($args.ok().unwrap(),)+))
    } else {
      None
    }
  });
}
