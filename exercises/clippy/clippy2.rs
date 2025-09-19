// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res = 42;
    let mut option = Some(12);  // 将 option 声明为可变
    while let Some(x) = option {
        res += x;
        option = None;  // 在循环中将 option 设为 None 来退出循环
    }
    println!("{}", res);  // 输出: 54
}
