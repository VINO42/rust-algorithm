mod stack_struct;
use stack_struct::stack::Stack;

fn main() {
    println!("Hello, world!");
     let mut st=Stack::new();
    st.push(1);
    st.push(2);
    println!("top:{},size:{}",st.top,st.data.len());
    st.push(3);
    println!("top:{},size:{}",st.top,st.data.len());
}
