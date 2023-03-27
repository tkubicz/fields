# Fields

Derive fields from structs and enums using derive macro.

## Example usage

```rust
use fields::Fields;

#[derive(Fields)]
#[fields(rename_all = "camelCase")]
struct SomeStruct {
  first_field: i32,
  second_field: i32,
  #[fields(skip)]
  third_field: i32,
  nested: Nested,
  #[fields(nested = false)]
  not_nested: NotNested;
}

#[derive(Fields)]
struct Nested {
  one: i32,
  two: i32,
}

struct NotNested {
  one: i32, 
  two: i32,
}


// Usage
fn main() {
  let fields = SomeStruct::fields();
  // Outputs Some("firstField", "secondField", "nested.one", "nested.two", "notNested")
  println!("{fields:?}"); 
}
```