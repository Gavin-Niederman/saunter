use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Interpolate)]
pub fn derive_interpolate(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let syn::Data::Struct(data) = ast.data else {
        panic!("Interpolate can only be derived for structs");
    };

    let ident = &ast.ident;

    let field_idents: Vec<_> = data.fields
        .iter()
        .cloned()
        .map(|f| (f.ident.unwrap(), f.ty))
        .map(|(ident, ty)| 
            quote! { #ident: <#ty as saunter::interpolate::Interpolate>::interpolate(&a.#ident, &b.#ident, t, &f) }
        )
        .collect();

    quote! {
        impl saunter::interpolate::Interpolate for #ident {
            fn interpolate(a: &Self, b: &Self, t: f32, f: impl Fn(f32) -> f32) -> Self {
                Self {
                    #(#field_idents),*,
                }
            }
        }
    }
    .into()
}
