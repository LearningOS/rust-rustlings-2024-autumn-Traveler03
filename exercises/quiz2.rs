pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 修正类型拼写错误
    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => {
                    let mut new_string = string.to_string();
                    for _ in 0..*n {
                        new_string += "bar";
                    }
                    output.push(new_string);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 使用 crate::my_module::transformer; 导入 transformer 函数
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello", Command::Uppercase),
            (" all roads lead to rome! ", Command::Trim),
            ("foo", Command::Append(1)),
            ("bar", Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}