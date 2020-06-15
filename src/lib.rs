use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

pub trait Storable: Any {}

pub struct Database {
    index: HashMap<TypeId, HashMap<u64, usize>>,
    items: Vec<Vec<u8>>,
    hash_state: RandomState,
}

impl Database {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            items: Vec::new(),
            hash_state: RandomState::new(),
        }
    }

    pub fn store<K, V>(&mut self, key: K, o: &V) -> Result<()>
    where
        K: std::hash::Hash,
        V: Any + Serialize + DeserializeOwned,
    {
        let data = bincode::serialize(o)?;
        self.items.push(data);
        let index = self.items.len() - 1;

        let mut hasher = self.hash_state.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        let map = self
            .index
            .entry(TypeId::of::<V>())
            .or_insert(HashMap::new());
        map.insert(hash, index);

        Ok(())
    }

    pub fn fetch<K, V>(&self, key: K) -> Result<V>
    where
        K: std::fmt::Debug + std::hash::Hash,
        V: Any + Serialize + DeserializeOwned,
    {
        if let Some(map) = self.index.get(&TypeId::of::<V>()) {
            let mut hasher = self.hash_state.build_hasher();
            key.hash(&mut hasher);
            let hash = hasher.finish();

            if let Some(index) = map.get(&hash) {
                let data = &self.items[*index];
                let res = bincode::deserialize(data)?;
                return Ok(res);
            }
        }
        Err(anyhow!("No such key: {:?}", key))
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct A {
    pub vala: u8,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct B {
    pub valb: u8,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn store_and_fetch() -> Result<()> {
        let mut db = Database::new();
        let a = A { vala: 1 };
        let b = B { valb: 2 };
        db.store(1, &a)?;
        db.store(1, &b)?;
        assert_eq!(a, db.fetch(1)?);
        assert_eq!(b, db.fetch(1)?);
        Ok(())
    }

    #[test]
    fn fetch_bad_key() -> Result<()> {
        let mut db = Database::new();
        let a = A { vala: 1 };
        db.store(1, &a)?;
        assert!(db.fetch::<u64, A>(2).is_err());
        Ok(())
    }

    #[test]
    fn fetch_bad_key_type() -> Result<()> {
        let mut db = Database::new();
        let a = A { vala: 1 };
        db.store::<u64, A>(1, &a)?;
        assert!(db.fetch::<u8, A>(1).is_err());
        Ok(())
    }
}
