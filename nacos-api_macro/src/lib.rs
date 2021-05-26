use proc_macro::TokenStream;
use syn::{DeriveInput, Data, DataStruct, Fields};
use quote::quote;
use heck::MixedCase;

#[proc_macro_derive(Dto)]
pub fn dto_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input)
        .expect(" -- parse err at derive dto ");

    let name = &ast.ident.clone();
    let fields = match &ast.data {
        Data::Struct(DataStruct {
                         fields: Fields::Named(fields),
                         ..
                     }) => &fields.named,
        _ => panic!(" -- analysis fields err at derive dto "),
    };

    let itm = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let mix = field_name.to_string().to_mixed_case();

        quote! {
            if let Some(s) = &self.#field_name {
                map.insert(#mix.to_string() , s.to_string());
            }
        }
    });

    TokenStream::from(quote! {
        impl Dto for #name {
            fn mapping(&self, map: &mut HashMap<String, String>) {
                #(
                    #itm
                )*
            }
        }
    })
}

