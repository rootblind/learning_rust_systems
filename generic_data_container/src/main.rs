use generic_data_container::storage::{Storage, StrorageBackend};

fn main() {
    let mut storage: Storage<i32> = Storage::new();
    for i in 1..=100 {
        storage.add(i);
    }

    storage.remove(0);
    let x = storage.retrieve(0).unwrap();
    let y = storage.retrieve(1).unwrap();

    println!("{} {}", x, y);
}
