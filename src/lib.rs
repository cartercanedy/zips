// Copyright (c) 2024 Carter Canedy <cartercanedy42@gmail.com>

use {
  proc_macro::TokenStream,
  syn::{parse_macro_input, Expr, Token},
  syn::parse::{Parse, ParseStream},
  syn::punctuated::Punctuated,
  quote::quote,
};

type ZipToken = Punctuated<Expr, Token![,]>;

macro_rules! extract_args {
  ($input:ident as $type:ty) => ({
      let args = parse_macro_input!($input as $type);
      let arg_names: Vec<syn::Ident> = 
        (0..args.args.len())
          .map(|i| syn::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site()))
          .collect();

      (args, arg_names)
  });
}

struct ZipArgs {
  args: ZipToken
}

impl Parse for ZipArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(Self {
      args: Punctuated::<Expr, Token![,]>::parse_terminated(input)?
    })
  }
}

/// Expands into a single `Option<(T [, T...])>::Some((...))` instance if all arguments
/// are instances of `Some((T [, T...]))`, else expands to `None`
///
/// ## Usage:
/// ```
/// let zipped_some = zip!(Some(0), Some(1));
/// assert_eq!(zipped_some, Some((0, 1)));
///
/// let zipped_none = zip!(Some(0), None);
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
  }.into()
}

struct ZipResultArgs {
  args: ZipToken
}

impl Parse for ZipResultArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(Self {
      args: Punctuated::<Expr, Token![,]>::parse_terminated(input)?
    })
  }
}

/// Expands into a single `Option<(T [, T...])>::Some((...))` instance if all arguments
/// are instances of `Some((T [, T...]))`, else expands to `None`
///
/// ## Usage:
/// ```
/// let zipped_some = zip!(Some(0), Some(1));
/// assert_eq!(zipped_some, Some((0, 1)));
///
/// let zipped_none = zip!(Some(0), None);
/// assert_eq!(zipped_none, None);
/// ```
#[proc_macro]
pub fn zip_result(input: TokenStream) -> TokenStream {
  let (ZipResultArgs { args }, arg_names) = extract_args!(input as ZipResultArgs);

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
  }.into()
}
