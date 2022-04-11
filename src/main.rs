pub mod queue;

pub mod stack_struct;

use stack_struct::stack::Stack;

use queue::Queue;

fn main() {
    println!("Hello, world!");
    test_stack();
    test_queue();
    let v = vec!["郭靖", "杨康", "令狐冲", "杨过", "东方不败", "风清扬"];
    let the_last_child = hot_potato(v, 156);
    println!("the last child survived is :{}", the_last_child);
}

pub fn test_stack() {
    let mut st = Stack::new();
    st.push(1);
    st.push(2);
    println!("top:{},size:{}", st.top, st.data.len());
    st.push(3);
    println!("top:{:?},size:{}", st.pop().unwrap(), st.data.len());
    println!("top:{:?},size:{}", st.pop().unwrap(), st.data.len());
    println!("top:{:?},size:{}", st.pop().unwrap(), st.data.len());
    println!("isEmpty:{},size:{}", st.is_empty(), st.data.len());
    let s = "(((((())))))";
    let s1 = "(((((()))))";
    println!("s is checkPass{}", check_brackets(s));
    println!("s1 is checkPass{}", check_brackets(s1));
}
/**
 * 使用stack 做到存储{} 的压弹栈 (((((())))))
 *
 * 左括号压栈 右括号弹栈
 * 匹配返回true 不匹配返回false
 * 类似回文检查 上海自来水来自海上
 */
fn check_brackets(par: &str) -> bool {
    let mut arr = Vec::new();
    for c in par.chars() {
        arr.push(c);
    }
    let mut balanc = true;
    let mut index = 0;

    let mut stack = Stack::new();

    while index < arr.len() && balanc {
        let ch = arr[index];

        if '(' == ch {
            stack.push(ch);
        } else {
            if stack.is_empty() {
                balanc = false;
            } else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }
    balanc && stack.is_empty()
}

pub fn test_queue() {
    let mut q1 = Queue::new(3);

    let a1 = q1.enqueue(1);
    let a2 = q1.enqueue(2);
    let a3 = q1.enqueue(3);

    if let Err(error) = q1.enqueue(4) {
        println!("error:{}", error);
    } else {
        println!("empty queue")
    }
    let b1 = q1.dequeue();
    println!("queue b1:{}", b1.unwrap());

    println!(
        "queue size:{},isEmpty:{},q:{:?}",
        q1.size(),
        q1.is_empty(),
        q1
    );
}
/**
 * 假设拿山芋的孩子始终在队列的前面。当拿到山芋的时候，这个孩子将先出队再入队，把
自己放在队列的最后，这相当于他把山芋传递给了下一个孩子，而那个孩子必须处于队首，所
以他自己出队，让出了队首的位置，自己加入了队尾。经过 num 次的出队入队后，前面的孩
子被永久移除。接着另一个周期开始，继续此过程，直到只剩下一个名字。
 */
pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut arr = Queue::new(names.len());
    for name in names {
        let _rm = arr.enqueue(name);
    }
    //当队中的孩子大于一个的时候一直循环游戏
    while arr.size() > 1 {
        //传递土豆
        for i in 0..num {
            //队列中的孩子先出队 然后再入队
            let n = arr.dequeue().unwrap();
            let n1 = arr.enqueue(n);
        }
        //出入队列 完成一周 删除队尾的孩子
        arr.dequeue();
    }
    arr.dequeue().unwrap()
}
