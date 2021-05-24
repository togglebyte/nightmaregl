pub enum Entry<T> {
    Occupied(T),
    Vacant(Option<usize>),
}

pub struct Entries<T> {
    inner: Vec<Entry<T>>,
    next: Option<usize>,
}

impl<T> Entries<T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            next: None,
        }
    }

    fn remove(&mut self, index: usize) {
        let mut entry = Entry::Vacant(self.next.take());
        self.next = Some(index);
        std::mem::swap(&mut entry, &mut self.inner[index]);
    }

    pub fn push(&mut self, value: T, node_id: usize) {
        let entry = Entry::Occupied(value);

        let index = match self.next.take() {
            Some(index) => {
                if let Entry::Vacant(next) = self.inner[index] {
                    self.next = next;
                    self.inner[index] = entry;
                }
            },
            None => self.inner.push(entry),
        };
    }
}
