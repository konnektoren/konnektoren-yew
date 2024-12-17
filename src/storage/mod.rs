//! The `storage` module provides a common interface for interacting with storage

/// The `profile_storage` module manages the storage of user profiles.
pub mod profile_storage;

/// The `Storage` trait defines a common interface for interacting with storage
/// systems that store and retrieve items. Each storage type must define its
/// name, item type, and implement basic CRUD operations.
pub trait Storage {
    /// The name of the storage, used to uniquely identify the storage instance.
    const NAME: &'static str;

    /// The type of items that this storage handles.
    type Item;

    /// Retrieves an item from storage by its `id`. Returns `None` if the item is not found.
    ///
    /// # Parameters
    ///
    /// - `id`: A string slice representing the unique identifier of the item.
    ///
    /// # Returns
    ///
    /// An `Option` containing the item if it exists, or `None` if not.
    fn get(&self, id: &str) -> Option<Self::Item>;

    /// Retrieves all items from storage as a vector.
    ///
    /// # Returns
    ///
    /// A `Vec` containing all stored items.
    fn get_all(&self) -> Vec<Self::Item>;

    /// Inserts a new item into storage.
    ///
    /// # Parameters
    ///
    /// - `item`: The item to be inserted into storage.
    fn insert(&mut self, item: Self::Item);

    /// Updates an existing item in storage.
    ///
    /// # Parameters
    ///
    /// - `item`: The item to be updated in storage.
    fn update(&mut self, item: Self::Item);

    /// Deletes an item from storage by its `id`.
    ///
    /// # Parameters
    ///
    /// - `id`: A string slice representing the unique identifier of the item to be deleted.
    fn delete(&mut self, id: &str);
}

// Re-export the `ProfileStorage` type for managing user profile data.
//pub use profile_storage::ProfileStorage;
