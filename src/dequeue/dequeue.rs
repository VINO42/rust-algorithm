//双端队列 混合线性队列
#[derive(Debug)]
pub struct Dequeue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Dequeue<T> {
    pub fn new(cap: usize) -> Dequeue<T> {
        Dequeue {
            cap: cap,
            data: Vec::new(),
        }
    }
    //末尾为队首 push
    pub fn add_head(&mut self, val: T) -> Result<(), String> {
        if Self::size(self) == self.cap {
            return Err(String::from("dequeue is full"));
        }
        self.data.push(val);
        Ok(())
    }
    //首部为队尾 insert
    pub fn add_tail(&mut self, val: T) -> Result<(), String> {
        if Self::size(self) == self.cap {
            return Err(String::from("dequeue is full"));
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        Self::size(self) == 0
    }
    //从队首移除数据
    pub fn remove_head(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop();
        }
        None
    }
    //从队尾移除数据
    pub fn remove_tail(&mut self) {
        if Self::size(&self) > 0 {
            self.data.remove(0);
        }
        None
    }
}
