use std::fmt::Display;

use proc_macro_error::abort;
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Lit, Meta, Token};

#[derive(Default)]
pub struct StructAttributes {
    pub rename_all: Option<RenameAll>,
}

pub enum RenameAll {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

impl RenameAll {
    pub fn rename(&self, value: &str) -> String {
        match self {
            RenameAll::Lower => value.to_ascii_lowercase().replace(['_', '-'], ""),
            RenameAll::Upper => value.to_ascii_uppercase().replace(['_', '-'], ""),
            RenameAll::Pascal => {
                let mut result = String::new();
                let mut capitalize = true;
                for char in value.to_ascii_lowercase().chars() {
                    if char == '_' {
                        capitalize = true;
                    } else if capitalize {
                        result.push(char.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        result.push(char);
                    }
                }
                result
            }
            RenameAll::Camel => {
                let pascal = Self::Pascal.rename(value);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            RenameAll::Snake => value.to_ascii_lowercase().replace('-', "_"),
            RenameAll::ScreamingSnake => value.to_ascii_uppercase().replace('-', "_"),
            RenameAll::Kebab => value.to_ascii_lowercase().replace('_', "-"),
            RenameAll::ScreamingKebab => value.to_ascii_uppercase().replace('_', "-"),
        }
    }
}

impl<'a> TryFrom<&'a str> for RenameAll {
    type Error = RenameAllError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "lowercase" => Ok(Self::Lower),
            "UPPERCASE" => Ok(Self::Upper),
            "PascalCase" => Ok(Self::Pascal),
            "camelCase" => Ok(Self::Camel),
            "snake_case" => Ok(Self::Snake),
            "SCREAMING_SNAKE_CASE" => Ok(Self::ScreamingSnake),
            "kebab-case" => Ok(Self::Kebab),
            "SCREAMING-KEBAB-CASE" => Ok(Self::ScreamingKebab),
            invalid_value => Err(RenameAllError { invalid_value }),
        }
    }
}

#[derive(Debug)]
pub struct RenameAllError<'a> {
    invalid_value: &'a str,
}

impl<'a> Display for RenameAllError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid value `{}`. Available options are: `lowercase`, `UPPERCASE`, `PascalCase`, \
             'camelCase', `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, \
             `SCREAMING-KEBAB-CASE`",
            self.invalid_value
        )
    }
}

impl<'a> std::error::Error for RenameAllError<'a> {}

pub fn parse_struct_attributes(attributes: &Vec<Attribute>) -> StructAttributes {
    let mut attrs = StructAttributes::default();

    for attr in attributes {
        if attr.path().is_ident("fields") {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .expect("Cannot parse attribute tokens");

            for meta in nested {
                match meta {
                    Meta::NameValue(value) if value.path.is_ident("rename_all") => {
                        if let Expr::Lit(expr_lit) = value.value {
                            match expr_lit.lit {
                                Lit::Str(str_lit) => {
                                    let rename_all: RenameAll =
                                        match str_lit.value().as_str().try_into() {
                                            Ok(result) => result,
                                            Err(e) => abort!(str_lit, e),
                                        };
                                    attrs.rename_all = Some(rename_all);
                                }
                                _ => {
                                    abort!(
                                        expr_lit.lit,
                                        "Attribute `rename_all` expects string literal as value"
                                    )
                                }
                            }
                        } else {
                            abort!(
                                value.value,
                                "Attribute `rename_all` expects literal as value"
                            );
                        }
                    }
                    other => {
                        let ident = other
                            .path()
                            .get_ident()
                            .expect("Cannot get identifier for unrecognized attribute");
                        abort!(ident, "Unrecognized attribute `{}`", ident)
                    }
                }
            }
        }
    }

    attrs
}
