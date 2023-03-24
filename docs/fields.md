Derive a list of fields for this structure.

# Marco parameters applied on the struct/enum level

These are the attributes that can be used on a structure or enum.

| Attribute  | Description                            | Type   | Optional | Default |
| ---------- | -------------------------------------- | ------ | -------- | ------- |
| rename_all | Rename all the fields in the structure | string | Y        | None    |

## Example

```rust
use fields::Fields;

#[derive(Fields)]
#[fields(rename_all = "camelCase")]
struct SomeStruct {
  first_field: i32,
  second_field: i32,
  third_field: i32,
}
```

# Macro parameters applied on the field level

These are the attributes that can be used on a field.

| Attribute | Description                                       | Type   | Optional | Default |
| --------- | ------------------------------------------------- | ------ | -------- | ------- |
| rename    | Rename the field                                  | string | Y        | None    |
| skip      | Skip the field from the final result              | bool   | Y        | false   |
| nested    | Recursively add fields from the nested structures | bool   | Y        | true    |

## Example 

```rust
use fields::Fields;

#[derive(Fields)]
struct SomeStruct {
  // Rename the field to `renamed`
  #[fields(rename = "renamed")] 
  original: String,
  // Skip the field `skipped`
  #[fields(skip)]
  skipped: bool,
  // By default, it will produce nested fields e.g [nested.field_one, nested.field_two]
  nested: NestedStruct,
  // This will produce only `not_nested`
  #[fields(nested = false)]
  not_nested: NestedStruct,
}

#[derive(Fields)]
struct NestedStruct {
  field_one: i32,
  field_two: i32,
}

```