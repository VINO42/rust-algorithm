mod stack_struct;
use stack_struct::stack::Stack;

fn main() {
    println!("Hello, world!");
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
