use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Fields, Type, Variant};

use crate::attributes::field::parse_field_attributes;
use crate::attributes::structure::StructAttributes;

pub(crate) fn parse_enum_variants(
    variants: &Punctuated<Variant, Comma>,
    struct_attrs: &StructAttributes,
) -> Vec<proc_macro2::TokenStream> {
    let mut token_stream = Vec::new();
    for variant in variants.iter() {
        let parsed_fields = parse_fields(&variant.fields, struct_attrs);
        token_stream.extend(parsed_fields);
    }
    token_stream
}

pub(crate) fn parse_fields(
    fields: &Fields,
    struct_attrs: &StructAttributes,
) -> Vec<proc_macro2::TokenStream> {
    let mut field_exprs = Vec::new();

    for field in fields.iter() {
        let field_name = field.ident.as_ref().map(|i| {
            if let Some(ref rename_all) = struct_attrs.rename_all {
                rename_all.rename(&i.to_string())
            } else {
                i.to_string()
            }
        });
        let field_type = &field.ty;
        let field_attrs = parse_field_attributes(&field.attrs);

        let nested_struct = match field_type {
            Type::Path(type_path) => Some(type_path),
            _ => None,
        };

        if !field_attrs.skip {
            match (field_name, nested_struct) {
                (Some(name), Some(type_path)) => {
                    let name = field_attrs.rename.unwrap_or(name);
                    let fields = if field_attrs.nested {
                        quote! { <#type_path as ::field_names::Fields>::fields() }
                    } else {
                        quote! { &None }
                    };

                    field_exprs.push(quote! { (Some(#name), #fields) })
                }
                (None, Some(type_path)) => {
                    let fields = if field_attrs.nested {
                        quote! { <#type_path as ::field_names::Fields>::fields() }
                    } else {
                        quote! { &None }
                    };

                    field_exprs.push(quote! { (None, #fields) })
                }
                (Some(name), None) => {
                    let name = field_attrs.rename.unwrap_or(name);
                    field_exprs.push(quote! { (Some(#name), None) })
                }
                (None, None) => {}
            }
        }
    }
    field_exprs
}
