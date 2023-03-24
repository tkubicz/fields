use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Lit, Meta, Token};

#[derive(Debug)]
pub(crate) struct FieldAttributes {
    pub skip: bool,
    pub rename: Option<String>,
    pub nested: bool,
}

impl Default for FieldAttributes {
    fn default() -> Self {
        Self {
            skip: false,
            rename: None,
            nested: true,
        }
    }
}

impl FieldAttributes {
    fn skip(&mut self) {
        self.skip = true;
    }

    fn rename(&mut self, new_name: String) {
        self.rename = Some(new_name);
    }

    fn nested(&mut self, value: bool) {
        self.nested = value;
    }
}

pub(crate) fn parse_field_attributes(attributes: &Vec<Attribute>) -> FieldAttributes {
    let mut attrs = FieldAttributes::default();

    for attr in attributes {
        if attr.path().is_ident("fields") {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .expect("Cannot parse attribute tokens");

            for meta in nested {
                match meta {
                    Meta::Path(path) if path.is_ident("skip") => attrs.skip(),
                    Meta::NameValue(value) if value.path.is_ident("rename") => {
                        if let Expr::Lit(expr_lit) = value.value {
                            match expr_lit.lit {
                                Lit::Str(str_lit) => {
                                    let new_name = str_lit.value();
                                    attrs.rename(new_name);
                                }
                                _ => panic!("Attribute `rename` expects string literal as value"),
                            }
                        } else {
                            panic!("Attribute `rename` expects literal as value");
                        }
                    }
                    Meta::NameValue(value) if value.path.is_ident("nested") => {
                        if let Expr::Lit(expr_lit) = value.value {
                            match expr_lit.lit {
                                Lit::Bool(bool_lit) => {
                                    let new_value = bool_lit.value();
                                    attrs.nested(new_value);
                                }
                                _ => panic!("Attribute `nested` expects bool literal as value"),
                            }
                        } else {
                            panic!("Attribute `nested` expects literal as value")
                        }
                    }

                    other => {
                        let ident = other
                            .path()
                            .get_ident()
                            .expect("Cannot get identifier for unrecognized attribute");
                        panic!("Unrecognized attribute `{}`", ident)
                    }
                }
            }
        }
    }

    attrs
}
