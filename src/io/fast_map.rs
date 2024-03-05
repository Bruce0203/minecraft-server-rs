pub struct FastMap<E> {
    vec: Vec<Option<E>>,
    index_queue: Vec<usize>,
}

impl<E> FastMap<E> {
    pub fn new() -> FastMap<E> {
        FastMap {
            vec: Vec::new(),
            index_queue: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> FastMap<E> {
        FastMap {
            vec: Vec::with_capacity(capacity),
            index_queue: Vec::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, index: usize) -> &mut E {
        unsafe { self.vec.get_unchecked_mut(index) }
            .as_mut()
            .unwrap()
    }

    pub fn add<F>(&mut self, f: F) -> std::io::Result<()>
    where
        F: FnOnce(usize) -> std::io::Result<E>,
    {
        if let Some(index) = self.index_queue.pop() {
            let elem = f(index)?;
            self.vec[index] = Some(elem);
        } else {
            let index = self.vec.len();
            let elem = f(index)?;
            self.vec.push(Some(elem));
        }
        Ok(())
    }

    pub fn remove(&mut self, index: usize) {
        self.index_queue.push(index);
        self.vec[index] = None;
    }
}
