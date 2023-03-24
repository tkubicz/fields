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

impl From<&str> for RenameAll {
    fn from(value: &str) -> Self {
        match value {
            "lowercase" => Self::Lower,
            "UPPERCASE" => Self::Upper,
            "PascalCase" => Self::Pascal,
            "camelCase" => Self::Camel,
            "snake_case" => Self::Snake,
            "SCREAMING_SNAKE_CASE" => Self::ScreamingSnake,
            "kebab-case" => Self::Kebab,
            "SCREAMING-KEBAB-CASE" => Self::ScreamingKebab,
            _ => panic!(
                "Invalid value. Available options are: `lowercase`, `UPPERCASE`, `PascalCase`, \
                 'camelCase', `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, \
                 `SCREAMING-KEBAB-CASE`"
            ),
        }
    }
}

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
                                    let rename_all: RenameAll = str_lit.value().as_str().into();
                                    attrs.rename_all = Some(rename_all);
                                }
                                _ => {
                                    panic!("Attribute `rename_all` expects string literal as value")
                                }
                            }
                        } else {
                            panic!("Attribute `rename_all` expects literal as value");
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
