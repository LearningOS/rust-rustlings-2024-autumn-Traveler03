# Error handling

Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Further information
- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)

## 学习
### ?运算符
- 运算符只能在**返回类型**是 Result 或 Option 的函数中使用。
- 使用 ? 运算符的函数需要在错误类型上实现 From trait，以便进行自动转换。

### 什么是From Trait
From trait 是一种让你可以轻松地将一种类型转换为另一种类型的工具。它定义了一个方法 from，用于从一种类型创建另一种类型。

### 显示使用return 嵌套里面要显示的使用return
- 提前返回 if else 分支里面
- 长函数 复杂逻辑
- 函数返回类型为()

### 自己定义的错误类型需要显示的实现要求的某些特性才能使用类似Box<dyn std::error::Error>