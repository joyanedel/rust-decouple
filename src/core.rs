use std::{env, str::FromStr};

pub struct Environment;

impl Environment {
    /// Retrieve the environment variable parsed as `T`
    /// Panic if variable is not found and default value is not provided
    pub fn from<T: FromStr>(var_name: &str, default: Option<T>) -> T {
        let value = env::var(var_name);
        if value.is_err() && default.is_none() {
            panic!("Couldn't find `{var_name}`");
        } else if let Some(default_value) = default {
            return default_value;
        }

        let value = value.unwrap();
        let parsed_value = T::from_str(&value);

        if let Ok(t) = parsed_value {
            t
        } else {
            panic!("Couldn't parse `{var_name}` = {value}")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::core::Environment;

    #[test]
    fn parse_env_var_u8_correctly() {
        env::set_var("RIGHT_VALUE", "123");
        let result: u8 = Environment::from("RIGHT_VALUE", None);
        assert_eq!(result, 123);
    }

    #[test]
    fn parse_env_var_f64_correctly() {
        env::set_var("RIGHT_F64_VALUE", "12.345");
        let result: f64 = Environment::from("RIGHT_F64_VALUE", None);
        assert_eq!(result, 12.345)
    }

    #[test]
    fn parse_not_set_env_var_with_default_value_correctly() {
        env::remove_var("NOT_SET_VALUE");
        let result: u8 = Environment::from("NOT_SET_VALUE", Some(42));
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic(expected = "Couldn't find `NOT_SET_VALUE_2`")]
    fn parse_not_set_env_var_with_no_default_value_panics() {
        env::remove_var("NOT_SET_VALUE_2");
        Environment::from::<u8>("NOT_SET_VALUE_2", None);
    }

    #[test]
    #[should_panic(expected = "Couldn't parse `WRONG_TYPED_VALUE` = 12r3")]
    fn parse_wrong_typed_value_panics() {
        env::set_var("WRONG_TYPED_VALUE", "12r3");
        Environment::from::<u8>("WRONG_TYPED_VALUE", None);
    }
}
