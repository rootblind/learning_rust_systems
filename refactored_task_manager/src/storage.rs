use std::collections::HashMap;

pub trait Storage<K, T> {
    fn insert(&mut self, key: K, item: T);
    fn retrieve(&self, key: K) -> Option<&T>;
    fn retrieve_mut(&mut self, key: K) -> Option<&mut T>;
    fn remove(&mut self, key: K) -> Option<T>;
    fn get_storage(&self) -> &HashMap<K, T>;
}

pub struct StorageBackend<K, T> {
    container: HashMap<K, T>,
}

impl<K, T> Storage<K, T> for StorageBackend<K, T>
where
    K: std::hash::Hash + Eq,
{
    fn insert(&mut self, key: K, item: T) {
        self.container.insert(key, item);
    }

    fn retrieve(&self, key: K) -> Option<&T> {
        self.container.get(&key)
    }

    fn retrieve_mut(&mut self, key: K) -> Option<&mut T> {
        self.container.get_mut(&key)
    }

    fn remove(&mut self, key: K) -> Option<T> {
        self.container.remove(&key)
    }

    fn get_storage(&self) -> &HashMap<K, T> {
        &self.container
    }
}

impl<K, T> StorageBackend<K, T>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            container: HashMap::new(),
        }
    }
}
