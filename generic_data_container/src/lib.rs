pub mod storage;

#[cfg(test)]
mod tests {
    use crate::storage::{Storage, StrorageBackend};

    #[test]
    fn is_working() {
        let mut store: Storage<i32> = Storage::new();
        assert!(store.is_empty());
        store.add(1);
        assert_eq!(store.len(), 1);
        store.add(2);
        let index1 = store.retrieve(1).unwrap();
        assert_eq!(index1, &2);
        let removed_from_index0 = store.remove(0);
        assert_eq!(removed_from_index0, 1);
    }
}
