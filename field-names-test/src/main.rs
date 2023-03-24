#![allow(dead_code)]

use std::collections::HashMap;

use field_names::Fields;

#[derive(Fields)]
#[fields(rename_all = "camelCase")]
struct TestStruct {
    ala_ma_kota: String,
    kot_ma_ale: String,
    kot_nie_ma_wcale: Vec<String>,
    #[fields(nested = false)]
    test_test: HashMap<u32, Variant1>,
}

#[derive(Fields)]
enum TestEnum {
    Variant1(Variant1),
    Variant2(Variant2),
    Vartiant3 { c: i32, d: i32 },
}

#[derive(Fields)]
#[fields(rename_all = "camelCase")]
struct Variant1 {
    a: i32,
}

#[derive(Fields)]
struct Variant2 {
    b: i32,
}

fn main() {
    println!("TestStruct: {:?}", TestStruct::fields());
    //println!("TestEnum: {:?}", TestEnum::fields());
}
