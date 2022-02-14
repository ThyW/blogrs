#[derive(Debug)]
pub enum ArgsError {
    UnknownArg(String),
    WrongUsage(String)
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownArg(e) => write!(f, "Unknown command line argument `{}`", e),
            Self::WrongUsage(e) => write!(f, "Wrong usage: {}", e)
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Help,
    HomePage(String),
    BlogDir(String),
}

#[derive(Debug)]
struct PreparsedCommand {
    name: String,
    value: Option<String>
}

impl PreparsedCommand {
    fn new<S: AsRef<str>>(input: S) -> Self {
        let mut input = input.as_ref();

        while input.starts_with("-") {
            input = input.strip_prefix("-").unwrap();
        }

        if input.contains("=") {
            let (first, second) = input.split_at(input.find("=").unwrap());
            return Self {
                name: first.to_string(),
                value: Some(second[1..].to_string())
            }
        } else {
            return Self {
                name: input.to_string(),
                value: None
            }
        }
    }

    fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

pub fn parse_args(input: &Vec<String>) -> crate::error::BlogrsResult<Vec<Command>> {
    let mut ret = Vec::new();
    let mut preparsed: Vec<PreparsedCommand> = Vec::new();

    for each in input {
        if each.starts_with("-") {
            preparsed.push(PreparsedCommand::new(&each))
        } else {
            return Err(ArgsError::UnknownArg(each.clone()).into())
        }
    }

    for command in preparsed {
        match command.name.as_str() {
            "help" | "h" => {
                ret.push(Command::Help)
            }
            "home_page" => {
                if command.has_value() {
                    ret.push(Command::HomePage(command.value.unwrap().clone()))
                } else {
                    return Err(ArgsError::WrongUsage(format!("--home_page takes an arguemnt, none were provided.")).into())
                }
            }
            "blog_dir" => {
                if command.has_value() {
                    ret.push(Command::BlogDir(command.value.unwrap().clone()))
                } else {
                    return Err(ArgsError::WrongUsage(format!("--blog_dir takes an arguemnt, none were provided.")).into())
                }
            }
            _ => return Err(ArgsError::UnknownArg(command.name.clone()).into())
        }
    }

    if ret.is_empty() {
        ret.push(Command::Help)
    }

    Ok(ret)
}
