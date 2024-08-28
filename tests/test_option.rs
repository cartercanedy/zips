use zips::zip;

fn return_some() -> Option<()> {
  Some(())
}

fn return_none() -> Option<()> {
  None
}

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
  let opt1 = return_none();
  let zipped = zip!(opt1);
  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_2_options_mixed() -> () {
  let opt1 = return_none();
  let opt2 = return_some();

  let zipped = zip!(opt1, opt2);

  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_3_options_mixed() -> () {
  let opt1 = return_none();
  let opt2 = return_some();
  let opt3 = return_some();

  let zipped = zip!(opt1, opt2, opt3);

  assert_eq!(zipped, None);
}

#[allow(unused_parens)]
#[test]
pub fn test_zip_some_unaliased_arg() -> () {
  let zipped = zip!(return_some());
  
  assert_eq!(zipped, Some((())));
}

#[test]
pub fn test_zip_none_unaliased_arg() -> () {
  let zipped = zip!(return_none());
  
  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_mixed_unaliased_args() -> () {
  let zipped = zip!(
    return_some(),
    return_none()
  );

  assert_eq!(zipped, None);
}

#[test]
pub fn test_zip_some_unaliased_args() -> () {
  let zipped = zip!(
    Some(1u32),
    Some("2"),
    Some(3usize)
  );
  
  assert_eq!(zipped, Some((1u32, "2", 3usize)));
}
