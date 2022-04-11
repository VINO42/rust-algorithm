#[derive(Debug)]
pub struct Queue<T> {
    capacity: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Queue<T> {
        Queue {
            capacity: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        Self::size(self) == 0
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if (Self::size(&self) == self.capacity) {
            return Err(String::from("queue is full"));
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
             return self.data.pop()
        }

        None
    }
}
