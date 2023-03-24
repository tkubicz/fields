use std::collections::HashMap;

pub use once_cell::sync::OnceCell;

pub trait Fields {
    fn fields() -> &'static Option<Vec<String>>;
}

macro_rules! impl_none_fields {
    ($type_:ident) => {
        impl Fields for $type_ {
            fn fields() -> &'static Option<Vec<String>> {
                &None
            }
        }
    };
}

impl<T: Fields> Fields for Option<T> {
    fn fields() -> &'static Option<Vec<String>> {
        T::fields()
    }
}

impl<T: Fields, E> Fields for Result<T, E> {
    fn fields() -> &'static Option<Vec<String>> {
        T::fields()
    }
}

impl<T: Fields> Fields for Vec<T> {
    fn fields() -> &'static Option<Vec<String>> {
        T::fields()
    }
}

impl<K, V: Fields> Fields for HashMap<K, V> {
    fn fields() -> &'static Option<Vec<String>> {
        V::fields()
    }
}

impl_none_fields!(String);
impl_none_fields!(usize);
impl_none_fields!(u8);
impl_none_fields!(u16);
impl_none_fields!(u32);
impl_none_fields!(u64);
impl_none_fields!(u128);
impl_none_fields!(i8);
impl_none_fields!(i16);
impl_none_fields!(i32);
impl_none_fields!(i64);
impl_none_fields!(i128);
impl_none_fields!(f32);
impl_none_fields!(f64);
