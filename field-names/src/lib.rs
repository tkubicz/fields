mod instances;

pub use field_names_derive::Fields;
pub use once_cell::sync::OnceCell;

pub trait Fields {
    fn fields() -> &'static Option<Vec<String>>;
}
