use crate::storage::Storage;
use gloo::storage::{LocalStorage, Storage as _};
use konnektoren_core::prelude::PlayerProfile;

#[derive(Debug, Default)]
pub struct ProfileStorage {}

impl Storage for ProfileStorage {
    const NAME: &'static str = "profile";

    type Item = PlayerProfile;

    fn get(&self, id: &str) -> Option<Self::Item> {
        let key = format!("{}:{}", Self::NAME, id);
        if let Ok(item) = LocalStorage::get::<String>(key) {
            serde_json::from_str(&item).unwrap_or_default()
        } else {
            None
        }
    }

    fn get_all(&self) -> Vec<Self::Item> {
        unimplemented!("We can not get all profiles")
    }

    fn insert(&mut self, item: Self::Item) {
        let key = format!("{}:{}", Self::NAME, item.id);
        let value = serde_json::to_string(&item).unwrap();
        LocalStorage::set(key, value).unwrap();
    }

    fn update(&mut self, item: Self::Item) {
        self.insert(item);
    }

    fn delete(&mut self, id: &str) {
        let key = format!("{}:{}", Self::NAME, id);
        LocalStorage::delete(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_profile_storage() {
        let mut storage = ProfileStorage::default();
        let profile = PlayerProfile {
            id: "123".to_string(),
            name: "Alice".to_string(),
            xp: 100,
        };

        storage.insert(profile.clone());
        let stored_profile = storage.get(&profile.id).unwrap();
        assert_eq!(profile, stored_profile);

        let updated_profile = PlayerProfile {
            id: "123".to_string(),
            name: "Alice".to_string(),
            xp: 200,
        };
        storage.update(updated_profile.clone());
        let stored_profile = storage.get(&profile.id).unwrap();
        assert_eq!(updated_profile, stored_profile);

        storage.delete(&profile.id);
        let stored_profile = storage.get(&profile.id);
        assert!(stored_profile.is_none());
    }
}
