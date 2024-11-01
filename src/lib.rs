// Copyright (c) 2024 Carter Canedy <cartercanedy42@gmail.com>

use {
  proc_macro::TokenStream,
  quote::quote,
  syn::parse::{Parse, ParseStream},
  syn::punctuated::Punctuated,
  syn::{parse_macro_input, Expr, Token},
};

type ZipToken = Punctuated<Expr, Token![,]>;

macro_rules! extract_args {
  ($input:ident as $type:ty) => {{
    let args = parse_macro_input!($input as $type);
    let arg_names: Vec<syn::Ident> = (0..args.args.len())
      .map(|i| syn::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site()))
      .collect();

    (args, arg_names)
  }};
}

struct ZipArgs {
  args: ZipToken,
}

impl Parse for ZipArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(Self {
      args: ZipToken::parse_terminated(input)?,
    })
  }
}

/// Expands into a single `Some((T1 [, T2...]))` instance if all arguments
/// are instances of `Some(<T>)`, else expands to `None`
///
/// ## Usage:
/// ```
/// use zips::zip;
///
/// let i: Option<i32> = Some(0);
/// let j: Option<usize> = Some(1usize);
/// let k: Option<usize> = None;
///
/// //  zipped_some: Option<(i32, usize)>
/// let zipped_some = zip!(i, j);
/// assert_eq!(zipped_some, Some((0, 1usize)));
///
/// //  zipped_none: Option<(i32, usize, usize)>
/// let zipped_none = zip!(i, j, k);
/// assert_eq!(zipped_none, None);
/// ```
#[proc_macro]
pub fn zip(input: TokenStream) -> TokenStream {
  let (ZipArgs { args }, arg_names) = extract_args!(input as ZipArgs);

  let args = args.into_iter();

  quote! {
    {
      #(let #arg_names = #args;)*
      if #(#arg_names.is_some() &&)* true {
        Some((#(#arg_names.unwrap()),*))
      } else {
        None
      }
    }
  }
  .into()
}

/// Expands into a single `Some((T1 [, T2...]))` instance if all arguments
/// are instances of `Ok(<T>)`, else expands to `None`
///
/// ## Usage:
/// ```
/// use zips::zip_result;
///
/// let i: Result<i32, &'static str> = Ok(1);
/// let j: Result<usize, &'static str> = Ok(0usize);
/// let k: Result<usize, &'static str> = Err("I'm an error!");
///
/// //  zipped_ok: Option<(i32, usize)>
/// let zipped_ok = zip_result!(i, j);
/// assert_eq!(zipped_ok, Some((1, 0usize)));
///
/// //  zipped_err: Option<(i32, usize, usize)>
/// let zipped_err = zip_result!(i, j, k);
/// assert_eq!(zipped_err, None);
/// ```
#[proc_macro]
pub fn zip_result(input: TokenStream) -> TokenStream {
  let (ZipArgs { args }, arg_names) = extract_args!(input as ZipArgs);

  let args = args.into_iter();

  quote! {
    {
      #(let #arg_names = #args;)*
      if #(#arg_names.is_ok() &&)* true {
        Some((#(#arg_names.ok().unwrap()),*))
      } else {
        None
      }
    }
  }
  .into()
}
