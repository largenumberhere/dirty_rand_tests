extern crate proc_macro;

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{ItemFn, parse_macro_input, ReturnType, Stmt};

type TokenStream1 = proc_macro::TokenStream;
type TokenStream2 = proc_macro2::token_stream::TokenStream;

type TokenTree1 = proc_macro::TokenTree;
type TokenTree2 = proc_macro2::TokenTree;

#[proc_macro_attribute]
/// Make a type alias to use inside of your function that refers to the output type
/// ## Example:
/// ```rust
/// #[macros::alias_return_type(TOutput)]
/// fn number() -> u32 {
///     return 2;
/// }
/// ```
/// is tranformed into...
/// ```rust
/// fn number() -> u32 {
///     type TOutput = u32;
///     return 2;
/// }
/// ```
///
/// This allows for convienient access to your output type while making refactors easier, requiring only the method header to be modified.
pub fn alias_return_type(attr :TokenStream1, body:TokenStream1) -> TokenStream1 {
    let type_alias_name = parse_macro_input!(attr as Ident);

    // Validate it is a function and turn it into a convienient wrapper
    let mut func = parse_macro_input!(body as ItemFn);

    // Grab the output or default
    let output_type = match func.sig.output {
        ReturnType::Default => {
            TokenStream2::new()
        }
        ReturnType::Type(_, ref type_) => {
            type_.to_token_stream()
        }
    };

    // Create new statment to add
    let new_data = quote!{
        type #type_alias_name = # output_type;
    };
    let new_statement : Stmt = syn::parse2(new_data).unwrap();

    // Add statement to function
    func.block.stmts.insert(0, new_statement);

    // return the function as a TokenStream1
    func.to_token_stream().into()
}