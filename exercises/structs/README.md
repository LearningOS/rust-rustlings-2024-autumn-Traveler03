# Structs

Rust has three struct types: a classic C struct, a tuple struct, and a unit struct.
## 经典C结构体
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
## 元组结构体
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```
## 单元结构体 用来表示类型的存在 不存储数据
```rust
struct AlwaysEqual;
```
## notes
- Rust的struct字段是public的
- Rust的struct字段是immutable的

## Further information

- [Structures](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
