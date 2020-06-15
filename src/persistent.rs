use crate::Database;

use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::any::Any;

pub trait Storable: Any {}

pub struct DatabasePersistent {
    inner: Database,
    items: Vec<Vec<u8>>,
}

impl DatabasePersistent {
    pub fn new() -> Self {
        Self {
            inner: Database::new(),
            items: Vec::new(),
        }
    }

    pub fn store<K, V>(&mut self, key: K, v: V) -> Result<()>
    where
        K: std::hash::Hash,
        V: Any + Serialize + DeserializeOwned,
    {
        let data = bincode::serialize(&v)?;
        self.items.push(data);
        self.inner.store(key, v);

        Ok(())
    }

    pub fn fetch<K, V>(&self, key: K) -> Result<V>
    where
        K: std::fmt::Debug + std::hash::Hash,
        V: Any + Serialize + DeserializeOwned,
    {
        if let Some(index) = self.inner.index::<_, V>(&key) {
            let data = &self.items[index];
            let res = bincode::deserialize(data)?;
            return Ok(res);
        }

        Err(anyhow!("No such key: {:?}", key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_key_different_values() -> Result<()> {
        const KEY: u64 = 1;
        let mut db = DatabasePersistent::new();
        let a: u64 = 1;
        let b: i64 = 2;
        db.store(KEY, a)?;
        db.store(KEY, b)?;
        assert_eq!(a, db.fetch(KEY)?);
        assert_eq!(b, db.fetch(KEY)?);
        Ok(())
    }
}
