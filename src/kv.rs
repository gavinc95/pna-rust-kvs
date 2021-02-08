use std::collections::HashMap;

/// `KvStore` stores key-value pairs in memory.
///
/// The internal store is a `HashMap`, and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Create a key-value store.
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Sets the key-value pair in the store. The key and value are aboth `Strings`.
    ///
    /// This will overwrite an existing entry.
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Get the string value of the given key.
    ///
    /// Return `None` if the key doesn't exist in the store.
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Remove an entry from the store for the given key.
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
