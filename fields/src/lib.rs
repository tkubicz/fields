mod instances;

pub use fields_derive::Fields;
pub use once_cell::sync::OnceCell;

pub trait Fields {
    fn fields() -> &'static Option<Vec<String>>;
}
