#[derive(Debug)]
pub struct Stack<T> {
   pub top: usize,
   pub data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    // 压栈
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }
    // 弹栈
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return Option::None;
        }
        self.top -= 1;
        self.data.pop()
    }
    //获取数据
    pub fn peek(&mut self) -> Option<&T> {
        if self.top == 0 {
            return Option::None;
        }
        self.data.get(self.top - 1)
    }
    //获取数据
    pub fn peek_by_index(&mut self, index: usize) -> Option<&T> {
        if self.top == 0 {
            return Option::None;
        }
        self.data.get(index)
    }
    // 是否为空
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0 && self.top == 0
    }
    //获取栈大小
    pub fn size(&mut self) -> usize {
        self.top
    }
}
