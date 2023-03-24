use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type, Variant};

#[proc_macro_derive(Fields)]
pub fn derive_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    let parsed_fields = match &input.data {
        Data::Struct(data) => parse_fields(&data.fields),
        Data::Enum(en) => parse_enum_variants(&en.variants),
        _ => panic!("Fields macro is only applicable to named structs or enums"),
    };

    let result = quote! {
        impl #impl_generics ::field_names::Fields for #name #type_generics #where_clause {
            fn fields() -> &'static Option<Vec<String>> {
                static INSTANCE: ::field_names::OnceCell<Option<Vec<String>>> = ::field_names::OnceCell::new();
                INSTANCE.get_or_init(|| {
                    let mut field_names = Vec::new();
                    #(
                        let (field_name, optional_fields) = #parsed_fields;
                        match optional_fields {
                            Some(fields) => {
                                field_names.extend(
                                    fields
                                        .iter()
                                        .map(|field| format!("{}.{}", field_name, field))
                                        .collect::<Vec<_>>()
                                );
                            },
                            None => {
                                field_names.push(field_name);
                            }
                        }
                    )*
                    Some(field_names)
                })
            }
        }
    };

    TokenStream::from(result)
}

fn parse_enum_variants(variants: &Punctuated<Variant, Comma>) -> Vec<proc_macro2::TokenStream> {
    let mut token_stream = Vec::new();
    for variant in variants.iter() {
        let parsed_fields = parse_fields(&variant.fields);
        token_stream.extend(parsed_fields);
    }
    token_stream
}

fn parse_fields(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let mut field_exprs = Vec::new();

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_type = &field.ty;

        let nested_struct = match field_type {
            Type::Path(type_path) => Some(type_path),
            _ => None,
        };

        match nested_struct {
            Some(type_path) => {
                field_exprs.push(quote! { (#field_name.to_string(), <#type_path as ::field_names::Fields>::fields()) })
            }
            None => field_exprs.push(quote! { (#field_name.to_string(), None) }),
        }
    }
    field_exprs
}
