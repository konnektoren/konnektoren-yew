use super::storage::Storage;
use super::storage_error::StorageError;
use async_trait::async_trait;
use gloo::storage::{LocalStorage as GlooLocalStorage, Storage as GlooStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq)]
pub struct LocalStorage {
    key_prefix: Option<String>,
}

impl LocalStorage {
    pub fn new(key_prefix: Option<&str>) -> Self {
        LocalStorage {
            key_prefix: key_prefix.map(|prefix| prefix.to_string()),
        }
    }

    fn prefixed_key(&self, key: &str) -> String {
        match &self.key_prefix {
            Some(prefix) => format!("{}:{}", prefix, key),
            None => key.to_string(),
        }
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn get<T: for<'de> Deserialize<'de> + Sync>(
        &self,
        key: &str,
    ) -> Result<Option<T>, StorageError> {
        let prefixed_key = self.prefixed_key(key);
        match GlooLocalStorage::get(&prefixed_key) {
            Ok(value) => Ok(Some(value)),
            Err(gloo::storage::errors::StorageError::KeyNotFound(_)) => Ok(None),
            Err(e) => Err(StorageError::AccessError(e.to_string())),
        }
    }

    async fn set<T: Serialize + Sync>(&self, key: &str, value: &T) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);
        GlooLocalStorage::set(&prefixed_key, value)
            .map_err(|e| StorageError::AccessError(e.to_string()))
    }

    async fn remove(&self, key: &str) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);
        GlooLocalStorage::delete(&prefixed_key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_test::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        field: String,
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_local_storage() {
        let storage = LocalStorage::new(Some("test"));
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
