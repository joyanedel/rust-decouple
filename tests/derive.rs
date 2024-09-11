use std::env;

use rust_decouple::Decouple;

#[test]
fn test_simple_struct_derive() {
    #[derive(Decouple)]
    struct Test {
        test_simple_struct_env: u8,
    }

    env::set_var("TEST_SIMPLE_STRUCT_ENV", "8");
    let result = Test::parse();

    assert!(result.is_ok_and(|v| v.test_simple_struct_env == 8))
}

#[test]
fn test_simple_struct_derive_fails() {
    #[derive(Decouple)]
    struct Test {
        _test_non_existing_env_var: u8,
    }

    env::remove_var("_TEST_NON_EXISTING_ENV_VAR");
    let result = Test::parse();

    assert!(result.is_err())
}
