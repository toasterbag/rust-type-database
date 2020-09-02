# A typed database for Rust
A database that can store values of different types with support for indexes of any types that are hashable. Support for persistance is planned.

The database comes in two flavours, one stores serialized values using serde and also supports persistence. 
The other flsvour stores values using boxed values and is around 20% faster.

The only requirements for the memory database are that keys are `Hash` and values are `Any`.

## Examples
Storing both signed and unsigned integers
```rust
fn main() {
    const KEY: u64 = 1;
    let mut db = Database::new();
    let a: u64 = 1;
    let b: i64 = 1;
    db.store(KEY, a);
    db.store(KEY, b);
    // Will fail if key is missing
    assert_eq!(&a, db.fetch_ref(KEY).unwrap());
    assert_eq!(&b, db.fetch_ref(KEY).unwrap());
    Ok(())
}
```
