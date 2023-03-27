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
  third_field: i32,
}

// Usage
fn main() {
  let fields = SomeStruct::fields();
  // Outputs Some("firstField", "secondField", "thirdField")
  println!("{fields:?}"); 
}
```