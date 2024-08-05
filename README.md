# Rust-Decouple

Rust decouple is a long term project that aims to mimic the functionality of the [python-decouple](https://pypi.org/project/python-decouple/) library

Due to the nature of the Rust language, the library will be implemented in a different way, but the goal is to provide a similar functionality. So far, the library is in a very early stage of development and is not ready for use.

The benefits of the rust version is that the cast is automatically done by the library, so you don't have to worry about it.

## Usage

### Basic usage

The most basic usage of the library is to get a variable from the environment, if it is not found, it will return an error.

```rs
use rust_decouple::macros::config;

...
let my_string: String = config!("VAR_NAME");
```

You can also specify a default value if the variable is not found

```rs
use rust_decouple::macros::config;

...
let my_string = config!("VAR_NAME", "default_value");
```

In this case, the variable type will be inferred from the default value.
If the default value is ambiguous, you can specify the type like this:

```rs
use rust_decouple::macros::config;

...
// The type is annotated by the user
let my_string: u8 = config!("VAR_NAME", 8);

// The type is inferred from the default value
let my_string = config!("VAR_NAME", 8u8);
```

Notice that this usage of default doesn't cover the case where the variable is found but is empty or invalid.

### Vectorized environment variables

You can also get a vector of values from the environment, the values should be separated by a comma without spaces in between.

```rs
use rust_decouple::macros::config_vec;

...
let my_vec: Vec<String> = config_vec!("VAR_NAME");
let my_vec = config_vec!("VAR_NAME", vec!["1", "2"]);
let my_vec: Vec<u8> = config_vec!("VAR_NAME", vec![1, 2]);
```
