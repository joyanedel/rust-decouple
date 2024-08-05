/// Macro for Environment parser
/// ## Usage
/// ```rs
/// let variable: u8 = config!("U8_VAR");
/// let variable_with_default: i32 = config!("NOT_DEFINED_I32_VAR", 0);
/// // Automatically inferred that `variable_with_default_and_type_inferred` is u8 due to the default value
/// let variable_with_default_and_type_inferred = config!("VAR", 0u8);
/// ```
#[macro_export]
macro_rules! config {
    ($var_name:expr, $default_value:expr) => {
        rust_decouple::core::Environment::from($var_name, Some($default_value))
    };
    ($var_name:expr) => {
        rust_decouple::core::Environment::from($var_name, None)
    };
}

/// Macro for Vector environment parser
/// ## Usage
/// ```rs
/// let variable: Vec<u8> = config!("VEC_U8_VAR");
/// let variable_with_default: Vec<i32> = config!("NOT_DEFINED_I32_VAR", vec![]);
/// // Automatically inferred that `variable_with_default_and_type_inferred` is u8 due to the default value
/// let variable_with_default_and_type_inferred = config!("VAR", vec![0u8]);
/// ```
#[macro_export]
macro_rules! config_vec {
    ($var_name:expr, $default_value:expr) => {
        rust_decouple::core::VecEnvironment::from($var_name, Some($default_value))
    };
    ($var_name:expr) => {
        rust_decouple::core::VecEnvironment::from($var_name, None);
    };
}
