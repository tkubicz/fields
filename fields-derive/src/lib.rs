//! [Fields] procedural macro
//!
//! `fields_derive` is a procedural macro crate for `fields`. It's a transient
//! dependency and should not be used directly.
mod attributes;
mod parse;

use attributes::structure::parse_struct_attributes;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[doc = include_str!("../../docs/fields.md")]
#[proc_macro_derive(Fields, attributes(fields))]
#[proc_macro_error]
pub fn derive_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();
    let struct_attrs = parse_struct_attributes(&input.attrs);

    let parsed_fields = match &input.data {
        Data::Struct(data) => parse::parse_fields(&data.fields, &struct_attrs),
        Data::Enum(en) => parse::parse_enum_variants(&en.variants, &struct_attrs),
        _ => abort!(
            input,
            "Fields macro is only applicable to named structs or enums"
        ),
    };

    let result = quote! {
        impl #impl_generics ::fields::Fields for #name #type_generics #where_clause {
            fn fields() -> &'static Option<std::collections::HashSet<String>> {
                static INSTANCE: ::fields::OnceCell<Option<std::collections::HashSet<String>>> = ::fields::OnceCell::new();
                INSTANCE.get_or_init(|| {
                    let mut field_names = std::collections::HashSet::new();
                    #({
                        let (field_name, optional_fields): (Option<&str>, &Option<std::collections::HashSet<String>>) = #parsed_fields;
                        match (field_name, optional_fields) {
                            (Some(name), Some(fields)) => {
                                field_names.extend(
                                    fields.iter().map(|field| format!("{}.{}", name, field))
                                );
                            },
                            (None, Some(fields)) => {
                                field_names.extend(
                                    fields.iter().map(|field| format!("{}", field))
                                );
                            }
                            (Some(name), None) => {
                                field_names.insert(name.to_string());
                            }
                            (None, None) => {}
                        }
                    })*
                    Some(field_names)
                })
            }
        }
    };

    TokenStream::from(result)
}
