//! Fields
//!
//! Fields is a library that gives you a possibility to automatically derive
//! field names of selected structures and enums with the use of a macro.
mod instances;

pub use fields_derive::Fields;
#[doc(hidden)]
pub use once_cell::sync::OnceCell;

pub trait Fields {
    fn fields() -> &'static Option<std::collections::HashSet<String>>;
}
