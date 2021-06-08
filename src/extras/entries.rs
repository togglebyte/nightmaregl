/// An entry that is either
/// vacant or occupied.
pub enum Entry<T> {
    /// Occupied entry
    Occupied(T),

    /// Vacant entry containing the 
    /// index of the next vacant entry.
    Vacant(Option<usize>),
}

/// Store [`Entry`]s.
/// When an entry is removed the slot is marked as vacant,
/// meaning entries never move, so it's safe to refer to an entry by
/// it's position.
///
/// ```
/// use nightmaregl::extras::Entries;
/// let mut entries = Entries::with_capacity(10);
/// entries.push("hello");
/// let index = entries.push("world");
/// assert_eq!(index, 1);
/// assert_eq!("hello", entries.remove(0));
/// let actual = entries[index];
/// assert_eq!("world", actual);
/// ```
pub struct Entries<T> {
    inner: Vec<Entry<T>>,
    next: Option<usize>,
}

impl<T> Entries<T> {
    /// Create a new instance of `Entries`.
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            next: None,
        }
    }

    /// Create a new instance of `Entries` with a set capacity.
    /// The capacity can change.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
            next: None,
        }
    }

    /// Remove an entry, returning the value
    pub fn remove(&mut self, index: usize) -> T {
        let mut entry = Entry::Vacant(self.next.take());
        self.next = Some(index);
        std::mem::swap(&mut entry, &mut self.inner[index]);
        match entry {
            Entry::Vacant(_) => panic!("trying to remove vacant entry"),
            Entry::Occupied(val) => val,
        }
    }

    /// Add an entry, returning it's index
    pub fn push(&mut self, value: T) -> usize {
        let entry = Entry::Occupied(value);

        match self.next.take() {
            Some(index) => {
                if let Entry::Vacant(next) = self.inner[index] {
                    self.next = next;
                    self.inner[index] = entry;
                }
                index
            },
            None => {
                let index = self.inner.len();
                self.inner.push(entry);
                index
            }
        }
    }
}

impl<T> std::ops::Index<usize> for Entries<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.inner.index(index) {
            Entry::Occupied(val) => val,
            Entry::Vacant(_) => panic!("No value here"),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Entries<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.inner.index_mut(index) {
            Entry::Occupied(val) => val,
            Entry::Vacant(_) => panic!("No value here"),
        }
    }
}
