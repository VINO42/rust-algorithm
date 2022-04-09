struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }
}
// 压栈
fn push(&mut self, val: T) {
    self.data.push(val);
    self.to += 1;
}
// 弹栈
fn pop(&mut self) -> Option<T> {
    if self.top == 0 {
        return Option::None;
    }
    self.top -= 1;
    self.data.pop;
}
//获取数据
fn peek(&mut self) -> Option<T> {
    if self.top == 0 {
        return Option::None;
    }
    self.data.get(self.top - 1)
}
//获取数据
fn peek(&mut self, index: usize) -> Option<T> {
    if self.top == 0 {
        return Option::None;
    }
    self.data.get(index)
}
// 是否为空
fn is_empty(&self) -> bool {
    self.data.len() == 0 && self.top == 0
}
//获取栈大小
fn size(&mut self) ->usize{
    self.top 
}
