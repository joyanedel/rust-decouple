# Rust-Decouple

Rust decouple is a long term project that aims to mimic the functionality of the [python-decouple](https://pypi.org/project/python-decouple/) library

Due to the nature of the Rust language, the library will be implemented in a different way, but the goal is to provide a similar functionality. So far, the library is in a very early stage of development and is not ready for use.

The benefits of the rust version is that the cast is automatically done by the library, so you don't have to worry about it.

## Usage

### Basic usage

The most basic usage of the library is to get a variable from the environment, if it is not found, it will return an error.

```rs
use rust_decouple::macros::config;

let my_string: String = config!("VAR_NAME");
```

You can also specify a default value if the variable is not found

```rs
use rust_decouple::macros::config;

let my_string = config!("VAR_NAME", "default_value");
```

In this case, the variable type will be inferred from the default value.
If the default value is ambiguous, you can specify the type like this:

```rs
use rust_decouple::macros::config;

// The type is annotated by the user
let my_string: u8 = config!("VAR_NAME", 8);

// The type is inferred from the default value
let my_u8 = config!("VAR_NAME", 8u8);
```

Notice that this usage of default doesn't cover the case where the variable is found but is empty or invalid.

#### Vectorized environment variables

You can also get a vector of values from the environment, the values should be separated by a comma without spaces in between.

```rs
use rust_decouple::macros::config_vec;

let my_vec: Vec<String> = config_vec!("VAR_NAME");
let my_vec = config_vec!("VAR_NAME", vec!["1", "2"]);
let my_vec: Vec<u8> = config_vec!("VAR_NAME", vec![1, 2]);
```

### Derived trait

You can also derive the `Decouple` trait for your structs, this will allow you to get the values from the environment in a more structured way as the example below:

```rs
use rust_decouple::Decouple;

#[derive(Decouple)]
struct Test {
    var_1: u8,
    var_2: Vec<i32>,
    var_3: Vec<String>,
}

fn main() {
    let env_vars = Test::parse();
    println!("{}", env_vars.var_1);
    println!("{:?}", env_vars.var_2);
    println!("{:?}", env_vars.var_3);
}
```

To use it, you need to enable the feature `derive` in your `Cargo.toml` file:

```toml
[dependencies]
rust_decouple = { version = "0.1", features = ["derive"] }
```
