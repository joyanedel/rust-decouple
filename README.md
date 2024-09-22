# Rust-Decouple

Rust decouple is a long term project that aims to mimic the functionality of the [python-decouple](https://pypi.org/project/python-decouple/) library

Due to the nature of the Rust language, the library will be implemented in a different way, but the goal is to provide a similar functionality. So far, the library is in a very early stage of development and is not ready for use.

The benefits of the rust version is that the cast is automatically done by the library, so you don't have to worry about it.

## Usage

### Basic usage

The most basic usage of the library is to define a struct with the variables you want to decouple and then call the `parse` method on it. The library will automatically try to parse the environment variables and return a struct with the values.

```rs
use rust_decouple::Decouple;

#[derive(Decouple)]
struct EnvVars {
    api_key: String,
}

fn main() {
    let constants = match EnvVars::parse() {
        Ok(v) => v,
        Err(e) => panic!("Error at parsing environment variables. Error: {e}"),
    };

    println!("My secret API KEY value is: {}", constants.api_key)
}
```

If you have not set the environment variable `API_KEY`, this example program will panic with the message
```sh
Error at parsing environment variables. Error: Couldn't find variable `API_KEY`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Once you set the environment variable, the program will print the value of the variable. For instance, if you set the variable `API_KEY` to `123456`, the output will be:
```sh
My secret API KEY value is 123456
```

**Note** that this crate does not provide a way to set environment variables, you should set them before running your program, and as you might have noted, the library will look for the environment variable with the same name as the struct field in uppercase.

## Advanced usage

### Simple default values

The library also provides a way to set default values for the variables but not using procedural macros in the current version. 

```rs
use rust_decouple::core::Environment;

fn main() {
    let api_key = Environment::from("API_KEY", Some("sample_api_key".to_string()));

    println!("My secret API KEY value is: {:?}", api_key)
}
```

In this example, the library will look for the environment variable `API_KEY` and if it is not set, it will use the default
value `sample_api_key`.

One possible output of this program will be:
```sh
My secret API KEY value is: Ok("sample_api_key")
```

The derive macro is based in this implementation, so anything you can do with the Decouple derive macro, you can do with the Environment struct.

### Vector environment variables

The library also provides a way to parse environment variables as vectors. The library will look for the environment variable with the same name as the struct field in uppercase and will split the value by commas.

```rs
use rust_decouple::Decouple;

#[derive(Decouple)]
struct EnvVars {
    api_keys: Vec<String>,
}

fn main() {
    let constants = match EnvVars::parse() {
        Ok(v) => v,
        Err(e) => panic!("Error at parsing environment variables. Error: {e}"),
    };

    println!("My secret API KEYS values are: {:?}", constants.api_keys);
}
```

If you set the environment variable `API_KEYS` to `123456,7891011`, the output will be:
```
My secret API KEYS values are: ["123456", "7891011"]
```

And you can do the same with the VecEnvironment struct:

```rs
use rust_decouple::core::VecEnvironment;

fn main() {
    let api_keys = VecEnvironment::from("API_KEYS", Some(vec!["sample_api_key".to_string()]));
    println!("My secret API KEYS values are: {:?}", api_keys);
}
```

And the output will be:
```sh
My secret API KEYS values are: Ok(["sample_api_key"])
```
