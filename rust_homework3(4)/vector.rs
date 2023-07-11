#[derive(Debug)]
struct Vector<T: Default> {
    data: Vec<T>,
}

impl<T: Clone + Default + std::fmt::Debug> Vector<T> {
    fn new() -> Self {
        Vector { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Vector { data: Vec::with_capacity(capacity) }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, T::default());
    }
}

fn main() {
    let mut vector: Vector<i32> = Vector::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    println!("Vector: {:?}", vector);

    let item = vector.pop();
    println!("Popped item: {:?}", item);

    let removed_item = vector.remove(0);
    println!("Removed item: {:?}", removed_item);

    let item = vector.get(0);
    println!("Item at index 0: {:?}", item);

    vector.resize(5);
    println!("Resized vector: {:?}", vector);
}
