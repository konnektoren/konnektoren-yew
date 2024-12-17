use super::storage::Storage;
use super::storage_error::StorageError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default, Clone)]
pub struct MemoryStorage {
    storage: Arc<RwLock<HashMap<String, String>>>,
}

impl PartialEq for MemoryStorage {
    fn eq(&self, other: &Self) -> bool {
        if let (Ok(self_guard), Ok(other_guard)) = (self.storage.read(), other.storage.read()) {
            *self_guard == *other_guard
        } else {
            false
        }
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn get<T: for<'de> Deserialize<'de> + Sync>(
        &self,
        key: &str,
    ) -> Result<Option<T>, StorageError> {
        let storage = self
            .storage
            .read()
            .map_err(|e| StorageError::AccessError(e.to_string()))?;
        if let Some(value) = storage.get(key) {
            let deserialized: T = serde_json::from_str(value)
                .map_err(|e| StorageError::AccessError(e.to_string()))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    async fn set<T: Serialize + Sync>(&self, key: &str, value: &T) -> Result<(), StorageError> {
        let serialized =
            serde_json::to_string(value).map_err(|e| StorageError::AccessError(e.to_string()))?;
        let mut storage = self
            .storage
            .write()
            .map_err(|e| StorageError::AccessError(e.to_string()))?;
        storage.insert(key.to_string(), serialized);
        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StorageError> {
        let mut storage = self
            .storage
            .write()
            .map_err(|e| StorageError::AccessError(e.to_string()))?;
        storage.remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        field: String,
    }

    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::default();
        let test_value = TestStruct {
            field: "value".to_string(),
        };

        storage.set("key", &test_value).await.unwrap();
        assert_eq!(
            storage.get::<TestStruct>("key").await.unwrap(),
            Some(test_value)
        );
        storage.remove("key").await.unwrap();
        assert_eq!(storage.get::<TestStruct>("key").await.unwrap(), None);
    }
}
