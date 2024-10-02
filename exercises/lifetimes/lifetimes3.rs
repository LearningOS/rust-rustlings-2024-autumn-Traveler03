// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

/*
生命周期参数确保Book结构体中的引用在整个结构体实例的生命周期内是有效的。
例如，如果我们在main函数末尾尝试使用book，编译器将确保name和title在book的生命周期内不会被释放。
*/
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
