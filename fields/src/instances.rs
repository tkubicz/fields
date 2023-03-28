use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use crate::Fields;

macro_rules! impl_none_fields {
    ($type_:ident) => {
        impl Fields for $type_ {
            fn fields() -> &'static Option<HashSet<String>> {
                &None
            }
        }
    };
}

impl<T: Fields> Fields for Option<T> {
    fn fields() -> &'static Option<HashSet<String>> {
        T::fields()
    }
}

impl<T: Fields, E> Fields for Result<T, E> {
    fn fields() -> &'static Option<HashSet<String>> {
        T::fields()
    }
}

impl<T: Fields> Fields for Vec<T> {
    fn fields() -> &'static Option<HashSet<String>> {
        T::fields()
    }
}

impl<T: Fields> Fields for VecDeque<T> {
    fn fields() -> &'static Option<HashSet<String>> {
        T::fields()
    }
}

impl<T: Fields> Fields for LinkedList<T> {
    fn fields() -> &'static Option<HashSet<String>> {
        T::fields()
    }
}

impl<K, V: Fields> Fields for HashMap<K, V> {
    fn fields() -> &'static Option<HashSet<String>> {
        V::fields()
    }
}

impl<K, V: Fields> Fields for BTreeMap<K, V> {
    fn fields() -> &'static Option<HashSet<String>> {
        V::fields()
    }
}

impl<T, S: Fields> Fields for HashSet<T, S> {
    fn fields() -> &'static Option<HashSet<String>> {
        S::fields()
    }
}

impl<T: Fields> Fields for BTreeSet<T> {
    fn fields() -> &'static Option<HashSet<String>> {
        T::fields()
    }
}

impl<T: Fields> Fields for BinaryHeap<T> {
    fn fields() -> &'static Option<HashSet<String>> {
        T::fields()
    }
}

impl<T: Fields> Fields for Box<T> {
    fn fields() -> &'static Option<std::collections::HashSet<String>> {
        T::fields()
    }
}

impl_none_fields!(bool);
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

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> Fields for chrono::DateTime<Tz> {
    fn fields() -> &'static Option<HashSet<String>> {
        &None
    }
}
