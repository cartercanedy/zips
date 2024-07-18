use {
  proc_macro::TokenStream,
  syn::{parse_macro_input, Expr, Token},
  syn::parse::{Parse, ParseStream},
  syn::punctuated::Punctuated,
  quote::quote,
};

struct ZipArgs {
  args: Punctuated<Expr, Token![,]>
}

impl Parse for ZipArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(Self {
      args: Punctuated::<Expr, Token![,]>::parse_terminated(input)?
    })
  }
}

/// Expands into a single `Option<(T [, T...])>::Some((...))` instance if all arguments
/// are instances of `Option<T>::Some(T)`, else expands to `Option<(T [, T...])>::None`
///
/// Usage:
/// ```
/// use zips::zip;
///
/// fn main() -> () {
///     let zipped = zip!(Some(0), Some(1));
///     assert_eq!(zipped, Some(0, 1));
/// }
/// ```
#[proc_macro]
pub fn zip(input: TokenStream) -> TokenStream {
  let ZipArgs { args } = parse_macro_input!(input as ZipArgs);
  let arg_names: Vec<_> =
    (0..args.len())
      .map(|i| syn::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site()))
      .collect();

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
