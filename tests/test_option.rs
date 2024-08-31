// Copyright (c) 2024 Carter Canedy <cartercanedy42@gmail.com>
use zips::zip;

#[allow(unused_parens)]
#[test]
pub fn test_zip_1_option() -> () {
  let opt = Some(1u32);
  let zipped = zip!(opt);
  assert_eq!(zipped, Some((1u32)));
}

#[test]
pub fn test_zip_2_options() -> () {
  let opt1 = Some(1u32);
  let opt2 = Some("2");

  let zipped = zip!(opt1, opt2);

  assert_eq!(zipped, Some((1u32, "2")));
}

#[test]
pub fn test_zip_3_options() -> () {
  let opt1 = Some(1u32);
  let opt2 = Some("2");
  let opt3 = Some(3usize);

  let zipped = zip!(opt1, opt2, opt3);

  assert_eq!(zipped, Some((1u32, "2", 3usize)));
}

#[test]
pub fn test_zip_1_option_none() -> () {
  let opt1 = Option::<()>::None;
  let zipped = zip!(opt1);
  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_2_options_mixed() -> () {
  let opt1 = Option::<()>::None;
  let opt2 = Option::<()>::None;

  let zipped = zip!(opt1, opt2);

  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_3_options_mixed() -> () {
  let opt1 = Option::<()>::None;
  let opt2 = Some(Option::<()>::None);
  let opt3 = Some(());

  let zipped = zip!(opt1, opt2, opt3);

  assert_eq!(zipped, None);
}

#[allow(unused_parens)]
#[test]
pub fn test_zip_some_unaliased_arg() -> () {
  let zipped = zip!(Some(()));

  assert_eq!(zipped, Some(()));
}

#[test]
pub fn test_zip_none_unaliased_arg() -> () {
  let zipped = zip!(Option::<()>::None);

  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_mixed_unaliased_args() -> () {
  let zipped = zip!(
    Some(()),
    Option::<()>::None
  );

  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_some_unaliased_args() -> () {
  let zipped = zip!(
    Some(1u32),
    Some(()),
    Some(3usize)
  );

  assert_eq!(zipped, Some((1u32, (), 3usize)));
}

#[test]
pub fn test_zip_some_nested_invokations() -> () {
  let a = Some(1i32);
  let b = Some(0usize);
  let c = Some(());

  let zipped = zip!(zip!(a, b), c);

  assert_eq!(zipped, Some(((1i32, 0usize), ())));
}

#[test]
pub fn test_zip_none_nested_invokations() -> () {
  let a = Some(1i32);
  let b = Some(0usize);
  let c = Option::<()>::None;

  let zipped = zip!(a, zip!(b, c));

  assert_eq!(zipped, None);
}
