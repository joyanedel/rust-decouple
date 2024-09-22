use std::{env, fmt::Display, str::FromStr};

pub struct Environment;
pub struct VecEnvironment;

#[derive(Debug, PartialEq)]
pub enum FromEnvironmentError {
    VariableNotFoundError(String),
    ParseVariableError(String, String),
}

impl Display for FromEnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VariableNotFoundError(v) => write!(f, "Couldn't find variable `{}`", v),
            Self::ParseVariableError(variable, value) => {
                write!(f, "Couldn't parse `{}` from value: '{}'", variable, value)
            }
        }
    }
}

impl Environment {
    /// Retrieve the environment variable parsed as `T`
    /// Panic if variable is not found and default value is not provided
    pub fn from<T: FromStr>(var_name: &str, default: Option<T>) -> Result<T, FromEnvironmentError> {
        let value = env::var(var_name);
        if value.is_err() && default.is_none() {
            return Err(FromEnvironmentError::VariableNotFoundError(
                var_name.to_string(),
            ));
        } else if let Some(default_value) = default {
            return Ok(default_value);
        }

        let value = value.unwrap();
        T::from_str(value.as_ref())
            .map_err(|_| FromEnvironmentError::ParseVariableError(var_name.to_string(), value))
    }
}

impl VecEnvironment {
    pub fn from<T: FromStr>(
        var_name: &str,
        default: Option<Vec<T>>,
    ) -> Result<Vec<T>, FromEnvironmentError> {
        let value = env::var(var_name);
        if value.is_err() && default.is_none() {
            return Err(FromEnvironmentError::VariableNotFoundError(
                var_name.to_string(),
            ));
        } else if let Some(default_value) = default {
            return Ok(default_value);
        }

        let value = value.unwrap();
        value
            .split(",")
            .map(T::from_str)
            .collect::<Result<Vec<T>, _>>()
            .map_err(|_| FromEnvironmentError::ParseVariableError(var_name.to_string(), value))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::core::{Environment, FromEnvironmentError, VecEnvironment};

    #[test]
    fn parse_env_var_u8_correctly() {
        env::set_var("RIGHT_VALUE", "123");
        let result: Result<u8, _> = Environment::from("RIGHT_VALUE", None);
        assert_eq!(result.unwrap(), 123);
    }

    #[test]
    fn parse_env_var_f64_correctly() {
        env::set_var("RIGHT_F64_VALUE", "12.345");
        let result: Result<f64, _> = Environment::from("RIGHT_F64_VALUE", None);
        assert_eq!(result.unwrap(), 12.345)
    }

    #[test]
    fn parse_not_set_env_var_with_default_value_correctly() {
        env::remove_var("NOT_SET_VALUE");
        let result: Result<u8, _> = Environment::from("NOT_SET_VALUE", Some(42));
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn parse_not_set_env_var_with_no_default_value_panics() {
        env::remove_var("NOT_SET_VALUE_2");
        let result = Environment::from::<u8>("NOT_SET_VALUE_2", None);
        assert!(result.is_err_and(
            |e| e == FromEnvironmentError::VariableNotFoundError("NOT_SET_VALUE_2".to_string())
        ))
    }

    #[test]
    fn parse_wrong_typed_value_panics() {
        env::set_var("WRONG_TYPED_VALUE", "12r3");
        let result = Environment::from::<u8>("WRONG_TYPED_VALUE", None);
        assert!(result.is_err_and(|e| e
            == FromEnvironmentError::ParseVariableError(
                "WRONG_TYPED_VALUE".to_string(),
                "12r3".to_string()
            )))
    }

    #[test]
    fn parse_vec_of_string_values_correctly() {
        env::set_var("VEC_STRING_VAL", "hello,world");
        let result: Vec<String> = VecEnvironment::from("VEC_STRING_VAL", None).unwrap();
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn parse_vec_of_usize_correctly() {
        env::set_var("VEC_USIZE_VAR", "0,1,2,3,4,5");
        let result: Vec<usize> = VecEnvironment::from("VEC_USIZE_VAR", None).unwrap();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn parse_not_set_vec_of_u8_with_default_value_correctly() {
        env::remove_var("VEC_U8_WITH_DEFAULT");
        let result = VecEnvironment::from("VEC_U8_WITH_DEFAULT", Some(vec![5u8, 42])).unwrap();
        assert_eq!(result, vec![5, 42]);
    }
}
