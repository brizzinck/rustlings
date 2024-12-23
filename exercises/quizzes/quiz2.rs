enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut result = Vec::new();
        for string in input {
            result.push(match string.1 {
                Command::Uppercase => string.0.to_uppercase(),
                Command::Trim => {
                    string.0.trim_start().trim_end().to_owned()
                }
                Command::Append(count) => {
                    let mut result = string.0;
                    for i in 0..count {
                        result.push_str("bar");
                    }
                    result
                }
            });
        }
        result
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
