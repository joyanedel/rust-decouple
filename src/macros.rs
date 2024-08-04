/// Macro for Environment parser
/// ## Usage
/// ```rs
/// let variable: u8 = config!("U8_VAR");
/// let variable_with_default: i32 = config!("NOT_DEFINED_I32_VAR", 0);
/// // Automatically inferred that `variable_with_default_and_type_inferred` is u8 due to the default value
/// let variable_with_default_and_type_inferred = config!("VAR", 0u8);
/// ``````
#[macro_export]
macro_rules! config {
    ($var_name:expr, $default_value:expr) => {
        Environment::from($var_name, Some($default_value))
    };
    ($var_name:expr) => {
        Environment::from($var_name, None)
    };
}
