#![allow(dead_code)]

use std::collections::HashMap;

use fields::Fields;

fn get_fields<T: Fields>() -> Vec<&'static str> {
    T::fields()
        .as_ref()
        .unwrap()
        .iter()
        .map(|f| f.as_str())
        .collect()
}

fn assert_fields(expected: &[&str], fields: &[&str]) {
    expected.iter().for_each(|e| assert!(fields.contains(e)));
}

#[test]
fn all_fields_are_included() {
    #[derive(Fields)]
    struct TestStruct {
        a: String,
        b: i32,
        c: f64,
        d: i64,
    }

    let fields: Vec<&str> = get_fields::<TestStruct>();
    assert_fields(&vec!["a", "b", "c"], &fields);
}

#[test]
fn single_level_nested_structs_are_supported() {
    #[derive(Fields)]
    struct Level0 {
        a: String,
        b: Level1,
    }

    #[derive(Fields)]
    struct Level1 {
        nested: String,
    }

    let fields: Vec<&str> = get_fields::<Level0>();
    assert_fields(&vec!["a", "b.nested"], &fields);
}

#[test]
fn deeply_nested_structs() {
    #[derive(Fields)]
    struct Level0 {
        a: u8,
        level_1: Level1,
    }

    #[derive(Fields)]
    struct Level1 {
        b: u16,
        level_2: Level2,
    }

    #[derive(Fields)]
    struct Level2 {
        c: u32,
        level_3: Level3,
    }

    #[derive(Fields)]
    struct Level3 {
        d: u64,
    }

    let fields: Vec<&str> = get_fields::<Level0>();
    assert_fields(
        &vec![
            "a",
            "level_1.b",
            "level_1.level_2.c",
            "level_1.level_2.level_3.d",
        ],
        &fields,
    );
}

#[test]
fn vec_derivation() {
    #[derive(Fields)]
    struct Test {
        x: String,
        y: String,
        vec: Vec<InnerVec>,
    }

    #[derive(Fields)]
    struct InnerVec {
        a: String,
        b: String,
        c: String,
    }

    let fields = get_fields::<Test>();
    assert_fields(&vec!["x", "y", "vec.a", "vec.b", "vec.c"], &fields);
}

#[test]
fn hash_map_derivation() {
    #[derive(Fields)]
    struct Test {
        x: String,
        y: HashMap<String, Inner>,
    }

    #[derive(Fields)]
    struct Inner {
        a: i32,
        b: i32,
    }

    let fields = get_fields::<Test>();
    assert_fields(&vec!["x", "y.a", "y.b"], &fields);
}

#[test]
fn parse_named_enum() {
    #[derive(Fields)]
    enum Test {
        Variant1 { a: String },
        Variant2 { b: String },
    }

    let fields = get_fields::<Test>();
    assert_fields(&vec!["a", "b"], &fields);
}

#[test]
fn parse_unnamed_enum() {
    #[derive(Fields)]
    enum Test {
        Variant1(Variant1),
        Variant2(Variant2),
    }

    #[derive(Fields)]
    struct Variant1 {
        a: i32,
    }

    #[derive(Fields)]
    struct Variant2 {
        b: i32,
    }

    let fields = get_fields::<Test>();
    assert_fields(&vec!["a", "b"], &fields);
}

#[test]
fn parse_mixed_enum() {
    #[derive(Fields)]
    enum TestEnum {
        Variant1(Variant1),
        Variant2 { c: i32, d: i32 },
    }

    #[derive(Fields)]
    struct Variant1 {
        a: i32,
        b: i32,
    }

    let fields = get_fields::<TestEnum>();
    assert_fields(&vec!["a", "b", "c", "d"], &fields);
}

#[test]
fn rename_field() {
    #[derive(Fields)]
    struct Test {
        #[fields(rename = "renamed")]
        original: String,
    }

    let fields = get_fields::<Test>();
    assert_fields(&vec!["renamed"], &fields);
}

#[test]
fn skip_fields() {
    #[derive(Fields)]
    struct Test {
        a: i32,
        #[fields(skip)]
        b: i32,
        #[fields(skip)]
        c: i32,
        d: i32,
    }

    let fields = get_fields::<Test>();
    assert_fields(&vec!["a", "d"], &fields);
}

#[test]
fn disable_nesting() {
    #[derive(Fields)]
    struct Test {
        a: i32,
        #[fields(nested = false)]
        b: Nested,
    }

    struct Nested {
        c: i32,
        d: i32,
    }

    let fields = get_fields::<Test>();
    assert_fields(&vec!["a", "b"], &fields);
}

#[test]
fn combine_field_attributes() {
    #[derive(Fields)]
    struct Test {
        #[fields(rename = "renamed")]
        original: i32,
        #[fields(skip)]
        skipped: i32,
        #[fields(nested = true, rename = "nested_renamed")]
        nested: Nested,
        #[fields(nested = false)]
        not_nested: NotNested,
    }

    #[derive(Fields)]
    struct Nested {
        #[fields(rename = "a_renamed")]
        a: i32,
        #[fields(rename = "b_renamed")]
        b: i32,
    }

    struct NotNested {
        c: i32,
        d: i32,
    }

    let fields = get_fields::<Test>();
    assert_fields(
        &vec![
            "renamed",
            "nested_renamed.a_renamed",
            "nested_renamed.b_renamed",
            "not_nested",
        ],
        &fields,
    );
}

#[test]
fn rename_all() {
    #[derive(Fields)]
    #[fields(rename_all = "camelCase")]
    struct Test {
        account_id: String,
        bank_id: String,
        wallet_id: String,
        some_longer_example_string: String,
    }

    let fields = get_fields::<Test>();
    assert_fields(
        &vec!["accountId", "bankId", "walletId", "someLongerExampleString"],
        &fields,
    );
}
