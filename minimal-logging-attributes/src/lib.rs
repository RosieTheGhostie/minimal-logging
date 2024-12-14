//! Minimalistic logging attributes to make development just a little bit easier.
//!
//! Provides a handful of procedural macro attributes to attach to functions and methods.
//! Some attributes only serve as wrappers, but others can transform declarations into definitions.
//!
//! # Known Issues
//!
//! ## Function Signatures in Generated Messages
//!
//! The representation of a function signature can look rather peculiar. For example, this function
//!
//! ```ignore
//! pub fn dot_product<T>(a: &[T], b: &[T]) -> T
//! where
//!     T: Copy + Default + Sized + AddAssign + Mul<Output = T>,
//! {
//!     // ...
//! }
//! ```
//!
//! would be represented as
//!
//! ```ignore
//! fn dot_product < T > (a : & [T], b : & [T]) -> T where T : Copy + Default + Sized + AddAssign + Mul < Output = T >,
//! ```
//!
//! First of all, `pub` magically disappeared. That's just because [`syn::Signature`] doesn't store
//! the visibility; higher-level structs like [`syn::ItemFn`] are in charge of that. I could pretty
//! easily fix this if I wanted to, but... eh.
//!
//! More importantly, there's way too much whitespace. This is an unfortunate side effect of the way
//! [`stringify`] handles [`syn::Signature`]. I could probably fix it if I wrote my own repr
//! function, but I don't understand [`syn`] nor [`quote`] well enough to do that.

extern crate minimal_logging_macros as macros;

mod fn_declaration;

use fn_declaration::FnDeclaration;
use function_wrapper::WrappedFn;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

/// Transforms a function declaration into a baseline definition that prints a warning message
/// and returns a default value.
///
/// This attribute is meant to serve a similar purpose as the [`todo!`] macro does, but it differs
/// in that calling the function will not result in a panic. This is useful if you haven't
/// implemented a function that's called relatively late in your program's logic. By marking that
/// function as `to_be_implemented`, you can allow the program to execute to its completion while
/// providing a reminder to implement the function. This is both safer and cleaner than commenting
/// out function calls and/or faking the results.
///
/// # Examples
///
/// ```rust
/// # use simple_logging_attributes::to_be_implemented;
/// #
/// #[to_be_implemented]
/// fn foo(a: i32, b: f32, c: &str);
///
/// assert_eq!(foo(5, 3.14159, "Hello, World!"), ());
/// ```
///
/// `foo` will expand to something like this:
///
/// ```rust
/// # use simple_logging_attributes::to_be_implemented;
/// #
/// fn foo(a: i32, b: f32, c: &str) {
///     simple_logging_macros::warnln!(
///         "`{}` is not yet implemented",
///         "fn foo(a : i32, b : f32, c : & str)",
///     );
/// }
/// #
/// # assert_eq!(foo(5, 3.14159, "Hello, World!"), ());
/// ```
///
/// You can also specify a default return value.
///
/// ```rust
/// # use simple_logging_attributes::to_be_implemented;
/// #
/// #[to_be_implemented(69)]
/// fn checksum(bytes: &[u8]) -> u64;
///
/// let bytes = b"Am I valid?";
/// assert_eq!(checksum(bytes), 69);
/// ```
///
/// The expansion looks pretty much the same, but the value is appended to the definition.
///
/// ```rust
/// # use simple_logging_attributes::to_be_implemented;
/// #
/// fn checksum(bytes: &[u8]) -> u64 {
///     simple_logging_macros::warnln!(
///         "`{}` is not yet implemented",
///         "fn checksum(bytes : & [u8]) -> u64",
///     );
///     69
/// }
/// #
/// # let bytes = b"Am I valid?";
/// # assert_eq!(checksum(bytes), 69);
/// ```
///
/// Explicit default values are required for any return type besides the unit type `()`.
///
/// # Notes
///
/// - I wanted to make the explicit syntax optional for return types that implement the [`Default`]
///   trait, but Rust doesn't have a way to do that as far as I can tell.
/// - I would've liked to automatically prefix each parameter with an underscore to avoid the
///   compiler's nagging, but I'm not familiar enough with [`syn`] and [`quote`] to know how to do
///   that.
///
/// # See Also
///
/// - [`simple_logging_macros::warnln`]
#[proc_macro_attribute]
pub fn to_be_implemented(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: TokenStream2 = args.into();
    let declaration = syn::parse::<FnDeclaration>(input).unwrap();
    let signature = &declaration.signature;
    let mut body: TokenStream2 = quote! {
        minimal_logging::macros::warnln!("`{}` is not yet implemented", stringify!(#signature));
    };
    body.extend(args);
    let function = declaration.create_definition(vec![], body).unwrap();
    quote! {
        #function
    }
    .into()
}

/// Adds a warning message to the beginning of a function definition indicating that the function is
/// a work-in-progress and should not be relied upon for accurate results.
///
/// This is similar in spirit to [`to_be_implemented`], but it serves more as a reminder that the
/// relevant functionality is incomplete. This makes debugging slightly easier, as if/when the
/// program abruptly starts producing weird outputs, the culprit is made abundantly clear.
///
/// # Examples
///
/// ```rust
/// # use simple_logging_attributes::wip;
/// #
/// #[wip]
/// fn checksum(bytes: &[u8]) -> u64 {
///     let mut checksum: u64 = 0;
///     for (i, &byte) in bytes.iter().enumerate() {
///         // TODO: Operate on the bytes.
///     }
///     checksum
/// }
///
/// let data = b"The quick brown fox jumped over the lazy dog";
/// assert_eq!(checksum(data), 0);
/// ```
///
/// `checksum` will expand to something like this:
///
/// ```rust
/// # use simple_logging_attributes::wip;
/// #
/// fn checksum(bytes: &[u8]) -> u64 {
///     simple_logging_macros::warnln!(
///         "`{}` is a work-in-progress",
///         "fn checksum(bytes : & [u8]) -> u64",
///     );
///     {
///         let mut checksum: u64 = 0;
///         for (i, &byte) in bytes.iter().enumerate() {}
///         checksum
///     }
/// }
/// #
/// # let data = b"The quick brown fox jumped over the lazy dog";
/// # assert_eq!(checksum(data), 0);
/// ```
#[proc_macro_attribute]
pub fn wip(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = syn::parse::<WrappedFn>(input).unwrap();
    let signature = &function.function.sig;
    function.set_pre_code(quote! {
        minimal_logging::macros::warnln!("`{}` is a work-in-progress", stringify!(#signature));
    });
    function.to_token_stream().into()
}
