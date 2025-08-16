# cutedogs 🐕

A simple procedural macro for generating comprehensive Rust documentation with minimal boilerplate.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cutedogs = "0.0.4"
```

## Usage

```rust
use cutedogs::document;

#[document(summary = "Adds two numbers together")]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Features

### Basic Documentation
```rust
#[document(
    summary = "Calculates the area of a rectangle",
    returns = "The area as a f64"
)]
fn area(width: f64, height: f64) -> f64 {
    width * height
}
```

### Parameters
```rust
#[document(
    summary = "Processes user data",
    params = {"name": "The user's name", "age": "The user's age in years"},
    returns = "A formatted string"
)]
fn process_user(name: &str, age: u32) -> String {
    format!("{} is {} years old", name, age)
}
```

### Examples
```rust
#[document(
    summary = "Divides two numbers",
    example = "let result = divide(10.0, 2.0);\nassert_eq!(result, 5.0);"
)]
fn divide(a: f64, b: f64) -> f64 {
    a / b
}
```

### Unimplemented Functions
```rust
#[document(unimplemented)]
fn future_feature() -> String {
    unimplemented!()
}

#[document(unimplemented = "Database integration not ready")]
fn connect_db() -> Result<(), String> {
    unimplemented!()
}
```

### Advanced Features
```rust
#[document(
    summary = "Advanced function with all features",
    params = {"data": "Input data to process"},
    returns = "Processed result",
    example = "let result = advanced_fn(vec![1, 2, 3]);",
    panics = "Panics if input is empty",
    safety = "Safe to use with any input",
    since = "1.0.0",
    see_also = "helper_fn, another_fn",
    note = "This function is experimental"
)]
fn advanced_fn(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().map(|x| x * 2).collect()
}
```

### Deprecation
```rust
#[document(
    summary = "Old function",
    deprecated = "Use new_function() instead",
    deprecated_since = "2.0.0"
)]
fn old_function() {}
```

## All Available Fields

| Field | Description | Example |
|-------|-------------|---------|
| `summary` | Brief description | `"Adds two numbers"` |
| `params` | Parameter descriptions | `{"x": "First number", "y": "Second number"}` |
| `returns` | Return value description | `"The sum as i32"` |
| `example` | Code example | `"let x = add(1, 2);"` |
| `panics` | When function panics | `"Panics if divisor is zero"` |
| `safety` | Safety information | `"Safe for all inputs"` |
| `since` | Version added | `"1.0.0"` |
| `see_also` | Related functions | `"multiply, subtract"` |
| `note` | Additional notes | `"Experimental feature"` |
| `deprecated` | Deprecation message | `"Use new_fn instead"` |
| `deprecated_since` | Version deprecated | `"2.0.0"` |
| `unimplemented` | Mark as unimplemented | `unimplemented` or `"Reason"` |

## License

MIT OR Apache-2.0
