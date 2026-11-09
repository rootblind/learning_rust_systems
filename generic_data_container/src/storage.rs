pub struct Storage<T> {
    container: Vec<T>,
}
pub trait StrorageBackend<T> {
    fn new() -> Storage<T>;

    fn add(&mut self, item: T);

    fn remove(&mut self, index: usize) -> T;

    fn retrieve(&self, index: usize) -> Option<&T>;
}

impl<T> StrorageBackend<T> for Storage<T> {
    fn new() -> Storage<T> {
        Storage {
            container: Vec::new(),
        }
    }

    fn add(&mut self, item: T) {
        self.container.push(item);
    }

    fn remove(&mut self, index: usize) -> T {
        self.container.swap_remove(index)
    }

    fn retrieve(&self, index: usize) -> Option<&T> {
        self.container.get(index)
    }
}
