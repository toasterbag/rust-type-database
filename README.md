# A typed database for Rust
A database that can store values of different types with support for indexes of any types that are hashable. Support for persistance is planned.

There are currently a few restraints on which types can be stored due to the plans on making the database persistable.
Keys need to be `Debug + Hash` while values need to be `Any + Serialize + DeserializeOwned`. In the future keys will also have to be `Serialize + DeserializeOwned` but there will also be a non-persistent version which will require neither keys nor values to be `Serialize + DeserializeOwned`.

## Examples
Storing both signed and unsigned integers
```rust
fn main() {
    const KEY: u64 = 1;
    let mut db = Database::new();
    let a: u64 = 1;
    let b: i64 = 1;
    db.store(KEY, a).expect("Failed to serialize data");
    db.store(KEY, b).expect("Failed to serialize data");
    // Will fail if key is missing
    assert_eq!(&a, db.fetch_ref(KEY).unwrap());
    assert_eq!(&b, db.fetch_ref(KEY).unwrap());
    Ok(())
}
```