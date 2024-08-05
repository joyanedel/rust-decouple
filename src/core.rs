use std::{env, str::FromStr};

pub struct Environment;
pub struct VecEnvironment;

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
        match T::from_str(&value) {
            Ok(t) => t,
            Err(_) => panic!("Couldn't parse `{var_name}` = {value}"),
        }
    }
}

impl VecEnvironment {
    pub fn from<T: FromStr>(var_name: &str, default: Option<Vec<T>>) -> Vec<T> {
        let value = env::var(var_name);
        if value.is_err() && default.is_none() {
            panic!("Couldn't find `{var_name}`");
        } else if let Some(default_value) = default {
            return default_value;
        }

        let value = value.unwrap();
        match value
            .split(",")
            .map(T::from_str)
            .collect::<Result<Vec<T>, _>>()
        {
            Ok(t) => t,
            Err(_) => panic!("Couldn't parse `{var_name}` = {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::core::{Environment, VecEnvironment};

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

    #[test]
    fn parse_vec_of_string_values_correctly() {
        env::set_var("VEC_STRING_VAL", "hello,world");
        let result: Vec<String> = VecEnvironment::from("VEC_STRING_VAL", None);
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn parse_vec_of_usize_correctly() {
        env::set_var("VEC_USIZE_VAR", "0,1,2,3,4,5");
        let result: Vec<usize> = VecEnvironment::from("VEC_USIZE_VAR", None);
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn parse_not_set_vec_of_u8_with_default_value_correctly() {
        env::remove_var("VEC_U8_WITH_DEFAULT");
        let result = VecEnvironment::from("VEC_U8_WITH_DEFAULT", Some(vec![5u8, 42]));
        assert_eq!(result, vec![5, 42]);
    }
}
