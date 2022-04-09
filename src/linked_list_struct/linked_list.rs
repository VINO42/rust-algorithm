// 节点 连 接用 Box 指针 （ 大小确定 ）， 因 为 确 定 大 小 才 能 分 配 内 存

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<Node> {
    // 前一个借点
    pub pre: Link<T>,
    // 后一个节点
    pub next: Link<T>,
    //链表长度
    pub length: usize,
}
pub struct Node<T> {
    element: T,
    next: Link<T>,
}
impl<T> LinkedList<T> {
    //构建一个链表
    pub fn new() -> Self {
        LinkedList {
            pre: None,
            next: None,
            length: 0,
        }
    }
    //是否为空
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    //链表长度
    pub fn length(&self) -> usize {
        self.length
    }
}
