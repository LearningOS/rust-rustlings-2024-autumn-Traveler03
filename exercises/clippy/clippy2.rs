// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    // 这里不能用while let会陷入无限循环
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
