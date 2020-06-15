use anyhow::{anyhow, Result};
use std::any::{Any, TypeId};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

pub trait Storable: Any {}

pub struct Database {
    index: HashMap<TypeId, HashMap<u64, usize>>,
    items_any: Vec<Box<dyn Any>>,
    hash_state: RandomState,
}

impl Database {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            items_any: Vec::new(),
            hash_state: RandomState::new(),
        }
    }

    pub(crate) fn hash<K>(&self, key: K) -> u64
    where
        K: std::hash::Hash,
    {
        let mut hasher = self.hash_state.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub(crate) fn index<K, V>(&self, key: K) -> Option<usize>
    where
        K: std::hash::Hash,
        V: Any,
    {
        if let Some(map) = self.index.get(&TypeId::of::<V>()) {
            let hash = self.hash(key);

            if let Some(index) = map.get(&hash) {
                return Some(*index);
            }
        }
        None
    }

    pub fn store<K, V>(&mut self, key: K, v: V) -> Result<()>
    where
        K: std::hash::Hash,
        V: Any,
    {
        self.items_any.push(Box::new(v));
        let index = self.items_any.len() - 1;

        let hash = self.hash(key);

        let map = self
            .index
            .entry(TypeId::of::<V>())
            .or_insert(HashMap::new());
        map.insert(hash, index);

        Ok(())
    }

    pub fn fetch_ref<K, V>(&self, key: K) -> Option<&V>
    where
        K: std::fmt::Debug + std::hash::Hash,
        V: Any,
    {
        if let Some(index) = self.index::<_, V>(key) {
            let val = &self.items_any[index];
            if let Some(res) = val.downcast_ref::<V>() {
                return Some(res);
            };
        }
        None
    }

    pub fn fetch_mut<K, V>(&mut self, key: K) -> Option<&mut V>
    where
        K: std::fmt::Debug + std::hash::Hash,
        V: Any,
    {
        if let Some(index) = self.index::<_, V>(key) {
            let val = self.items_any.get_mut(index).unwrap();
            if let Some(res) = val.downcast_mut::<V>() {
                return Some(res);
            };
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn same_key_different_values() -> Result<()> {
        const KEY: u64 = 1;
        let mut db = Database::new();
        let a: u64 = 1;
        let b: i64 = 2;
        db.store(KEY, a.clone())?;
        db.store(KEY, b.clone())?;
        assert_eq!(&a, db.fetch_ref(KEY).unwrap());
        assert_eq!(&b, db.fetch_ref(KEY).unwrap());
        Ok(())
    }

    #[test]
    fn fetch_missing_key() -> Result<()> {
        const KEY: u64 = 1;
        const OTHER_KEY: u64 = 2;
        let mut db = Database::new();
        let a: u64 = 1;
        db.store(KEY, a)?;

        let res = db.fetch_ref::<_, u64>(OTHER_KEY);
        assert!(res.is_none());
        Ok(())
    }

    #[test]
    fn fetch_missing_key_type() -> Result<()> {
        const KEY: u64 = 1;
        const OTHER_KEY: u8 = 1;
        let mut db = Database::new();
        let a: u64 = 1;
        db.store(KEY, a)?;

        let res = db.fetch_ref::<_, u64>(OTHER_KEY);
        assert!(res.is_none());
        Ok(())
    }

    #[test]
    fn fetch_ref() -> Result<()> {
        const KEY: u64 = 1;
        let mut db = Database::new();
        let a: u64 = 1;
        db.store(KEY, a.clone())?;
        assert_eq!(&a, db.fetch_ref(KEY).unwrap());
        Ok(())
    }

    #[test]
    fn fetch_mut() -> Result<()> {
        const KEY: u64 = 1;
        let mut db = Database::new();
        let a: u64 = 1;
        let b: u64 = 2;
        db.store(KEY, a.clone())?;

        {
            if let Some(i) = db.fetch_mut::<_, u64>(KEY) {
                *i = b
            }
        }

        assert_eq!(&b, db.fetch_ref(KEY).unwrap());
        Ok(())
    }
}
