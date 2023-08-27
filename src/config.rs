#[derive(Debug)]
pub struct Config {
    pub size: u32,
    pub without_numbers: bool,
    pub without_symbols: bool,
    pub without_letters: bool,
    pub without_uppercase: bool,
    pub without_lowercase: bool,
    pub strong: bool,
}

impl Config {
    const DEFAULT_SIZE: u32 = 8;
    const DEFAULT_WITHOUT_NUMBERS: bool = false;
    const DEFAULT_WITHOUT_SYMBOLS: bool = false;
    const DEFAULT_WITHOUT_LETTERS: bool = false;
    const DEFAULT_WITHOUT_UPPERCASE: bool = false;
    const DEFAULT_WITHOUT_LOWERCASE: bool = false;
    const DEFAULT_STRONG: bool = false;

    const SIZE_TYPE_ERROR_MESSAGE: &str = "The size must be an integer";
    const NO_CATEGORIES_ERROR_MESSAGE: &str = "At least one category must be included";
    const SIZE_STRONG_ERROR_MESSAGE: &str =
        "The size must be at least 8 characters for the password to be strong";

    const SIZE_FLAG: &str = "--size";
    const WITHOUT_NUMBERS_FLAG: &str = "--without-numbers";
    const WITHOUT_SYMBOLS_FLAG: &str = "--without-symbols";
    const WITHOUT_LETTERS_FLAG: &str = "--without-letters";
    const WITHOUT_UPPERCASE_FLAG: &str = "--without-uppercase";
    const WITHOUT_LOWERCASE_FLAG: &str = "--without-lowercase";
    const STRONG_FLAG: &str = "--strong";

    pub fn build<'a>(args: Vec<String>) -> Result<Config, String> {
        let mut size: u32 = Self::DEFAULT_SIZE;
        let mut without_numbers: bool = Self::DEFAULT_WITHOUT_NUMBERS;
        let mut without_symbols: bool = Self::DEFAULT_WITHOUT_SYMBOLS;
        let mut without_letters: bool = Self::DEFAULT_WITHOUT_LETTERS;
        let mut without_uppercase: bool = Self::DEFAULT_WITHOUT_UPPERCASE;
        let mut without_lowercase: bool = Self::DEFAULT_WITHOUT_LOWERCASE;
        let mut strong: bool = Self::DEFAULT_STRONG;

        let mut previous_arg_is_flag = false;

        for i in 1..args.len() {
            if args[i] == Self::SIZE_FLAG {
                previous_arg_is_flag = true;
                size = match args[i + 1].parse() {
                    Ok(size) => size,
                    Err(_) => return Err(Self::SIZE_TYPE_ERROR_MESSAGE.to_string()),
                };
            } else if args[i] == Self::WITHOUT_NUMBERS_FLAG {
                previous_arg_is_flag = true;
                without_numbers = Self::get_flag_value(&args, i)?;
            } else if args[i] == Self::WITHOUT_SYMBOLS_FLAG {
                previous_arg_is_flag = true;
                without_symbols = Self::get_flag_value(&args, i)?;
            } else if args[i] == Self::WITHOUT_LETTERS_FLAG {
                previous_arg_is_flag = true;
                without_letters = Self::get_flag_value(&args, i)?;
            } else if args[i] == Self::WITHOUT_UPPERCASE_FLAG {
                previous_arg_is_flag = true;
                without_uppercase = Self::get_flag_value(&args, i)?;
            } else if args[i] == Self::WITHOUT_LOWERCASE_FLAG {
                previous_arg_is_flag = true;
                without_lowercase = Self::get_flag_value(&args, i)?;
            } else if args[i] == Self::STRONG_FLAG {
                previous_arg_is_flag = true;
                strong = Self::get_flag_value(&args, i)?;
            } else if previous_arg_is_flag == false {
                return Err(format!("Unknown argument: {}", args[i]));
            } else {
                previous_arg_is_flag = false;
            }
        }

        if without_numbers
            && without_symbols
            && (without_letters || (without_uppercase && without_lowercase))
        {
            return Err(Self::NO_CATEGORIES_ERROR_MESSAGE.to_string());
        }

        if size < 8 && strong {
            return Err(Self::SIZE_STRONG_ERROR_MESSAGE.to_string());
        }

        Ok(Config {
            size,
            without_letters,
            without_lowercase,
            without_numbers,
            without_symbols,
            without_uppercase,
            strong,
        })
    }

    fn is_flag(flags: &Vec<String>, i: usize) -> bool {
        if i >= flags.len() {
            return true; // If there is no value after the flag, it is considered as true
        }

        if flags[i].starts_with("--") {
            return true;
        }
        false
    }

    fn get_flag_value(args: &Vec<String>, i: usize) -> Result<bool, String> {
        if Self::is_flag(&args, i + 1) {
            return Ok(true);
        } else {
            return match args[i + 1].as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(format!(
                    "The value of {} flag must be true or false",
                    args[i]
                )),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_config_build {
        ($($name: ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (args, expected) = $value;

                    let config = Config::build(args).unwrap();

                    assert_eq!(config.size, expected[0].parse().unwrap());
                    assert_eq!(config.without_numbers, expected[1].parse().unwrap());
                    assert_eq!(config.without_symbols, expected[2].parse().unwrap());
                    assert_eq!(config.without_letters, expected[3].parse().unwrap());
                    assert_eq!(config.without_uppercase, expected[4].parse().unwrap());
                    assert_eq!(config.without_lowercase, expected[5].parse().unwrap());
                    assert_eq!(config.strong, expected[6].parse().unwrap());
                }
            )*
        }
    }

    test_config_build! {
        test_config_build_1: (vec![], vec![Config::DEFAULT_SIZE.to_string(), Config::DEFAULT_WITHOUT_NUMBERS.to_string(), Config::DEFAULT_WITHOUT_SYMBOLS.to_string(), Config::DEFAULT_WITHOUT_LETTERS.to_string(), Config::DEFAULT_WITHOUT_UPPERCASE.to_string(), Config::DEFAULT_WITHOUT_LOWERCASE.to_string(), Config::DEFAULT_STRONG.to_string()]),
        test_config_build_2: (vec![
            "password_generator".to_string(),
            "--size".to_string(),
            "10".to_string(),
            "--without-symbols".to_string(),
            "--without-letters".to_string(),
            "--without-uppercase".to_string(),
            "--without-lowercase".to_string(),
            "--strong".to_string(),
        ], vec![10.to_string(), false.to_string(), true.to_string(), true.to_string(), true.to_string(), true.to_string(), true.to_string()]),
        test_config_build_3: (vec![
            "password_generator".to_string(),
            "--without-numbers".to_string(),
            "--without-symbols".to_string(),
        ], vec![Config::DEFAULT_SIZE.to_string(), true.to_string(), true.to_string(), Config::DEFAULT_WITHOUT_LETTERS.to_string(), Config::DEFAULT_WITHOUT_UPPERCASE.to_string(), Config::DEFAULT_WITHOUT_LOWERCASE.to_string(), Config::DEFAULT_STRONG.to_string()]),
    }
}
