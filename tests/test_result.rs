// Copyright (c) 2024 Carter Canedy <cartercanedy42@gmail.com>
use zips::zip_result;

#[allow(unused_parens)]
#[test]
pub fn test_result_1_arg() -> () {
  let res1: Result<u32, ()> = Ok(1u32);
  let zipped = zip_result!(res1);

  assert_eq!(zipped, Some((1u32)));
}

#[test]
pub fn test_result_2_args() -> () {
  let res1: Result<u32, ()> = Ok(1u32);
  let res2: Result<&'static str, ()> = Ok("2");

  let zipped = zip_result!(res1, res2);

  assert_eq!(zipped, Some((1u32, "2")));
}

#[test]
pub fn test_result_3_args() -> () {
  let res1: Result<u32, ()> = Ok(1u32);
  let res2: Result<&'static str, ()> = Ok("2");
  let res3: Result<i32, ()> = Ok(3i32);

  let zipped = zip_result!(res1, res2, res3);

  assert_eq!(zipped, Some((1u32, "2", 3i32)));
}

#[allow(unused_parens)]
#[test]
pub fn test_result_ok_unaliased_args() -> () {
  fn return_ok() -> Result<i32, ()> {
    Ok(1i32)
  }

  let zipped = zip_result!(return_ok());

  assert_eq!(zipped, Some((1i32)));
}

#[test]
pub fn test_result_err_unaliased_args() -> () {
  fn return_err() -> Result<(), ()> {
    Err(())
  }

  let zipped = zip_result!(return_err());

  assert_eq!(zipped, None);
}

#[test]
pub fn test_result_1_arg_err() -> () {
  let res1: Result<i32, ()> = Err(());
  let zipped = zip_result!(res1);

  assert_eq!(zipped, None);
}

#[test]
pub fn test_result_2_args_mixed() -> () {
  let res1: Result<i32, ()> = Ok(1i32);
  let res2: Result<&'static str, ()> = Err(());

  let zipped = zip_result!(res1, res2);

  assert_eq!(zipped, None);
}

#[test]
pub fn test_result_3_args_mixed() -> () {
  let res1: Result<u32, ()> = Ok(1u32);
  let res2: Result<&'static str, ()> = Err(());
  let res3: Result<i32, ()> = Ok(3i32);

  let zipped = zip_result!(res1, res2, res3);

  assert_eq!(zipped, None);
}

#[test]
pub fn test_result_ok_nested_invokations() -> () {
  type Res = Result<(), ()>;
  let a = Res::Ok(());
  let b = Res::Ok(());
  let c = Result::<i32, ()>::Ok(1i32);

  let zipped = zip_result!(
    Result::<_, ()>::Ok(zip_result!(a, b).unwrap()),
    c
  );

  assert_eq!(zipped, Some((((), ()), 1i32)));
}

#[test]
pub fn test_result_err_nested_invokations() -> () {
  type Res = Result<(), ()>;
  let a = Res::Ok(());
  let b = Res::Ok(());
  let c = Result::<(), i32>::Err(1i32);

  let zipped = zip_result!(
    Result::<_, ()>::Ok(zip_result!(a, b).unwrap()),
    c
  );

  assert_eq!(zipped, None);
}
