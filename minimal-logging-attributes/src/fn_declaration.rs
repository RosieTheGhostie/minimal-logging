use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Block, Error as SynError, ItemFn, Result as SynResult, Signature, Token, Visibility,
};

pub struct FnDeclaration {
    pub visibility: Visibility,
    pub signature: Signature,
    pub _semi: Token![;],
}

impl Parse for FnDeclaration {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.is_empty() {
            return Err(SynError::new(input.span(), "Expected function declaration"));
        }
        let visibility: Visibility = input.parse().unwrap_or(Visibility::Inherited);
        let signature: Signature = input.parse()?;
        let semi: Token![;] = input.parse()?;
        Ok(FnDeclaration {
            visibility,
            signature,
            _semi: semi,
        })
    }
}

impl FnDeclaration {
    pub fn create_definition(self, attrs: Vec<Attribute>, body: TokenStream2) -> SynResult<ItemFn> {
        let body = quote! { { #body } };
        let block: Box<Block> = syn::parse(body.into())?;
        Ok(ItemFn {
            attrs,
            vis: self.visibility,
            sig: self.signature,
            block,
        })
    }
}
