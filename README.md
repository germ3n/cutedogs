# cutedogs 🐕

**Programmatic Rust documentation** - Write docs like code, not comments.

cutedogs transforms function documentation from scattered comment blocks into structured, attribute-driven declarations. Instead of writing traditional doc comments, you define documentation as structured data using Rust attributes - making it type-safe, IDE-friendly, and programmatically verifiable.

## Why cutedogs?

### Traditional rustdoc (comment-based)
```rust
/// Calculates the area of a rectangle
/// 
/// # Parameters
/// * `width` - The width of the rectangle 
/// * `height` - The height of the rectangle
/// 
/// # Returns
/// The area as a f64
/// 
/// # Example
/// ```rust
/// let area = calculate_area(5.0, 3.0);
/// assert_eq!(area, 15.0);
/// ```
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

### cutedogs (attribute-driven)
```rust
use cutedogs::document;

#[document(
    summary = "Calculates the area of a rectangle",
    params = {"width": "The width of the rectangle", "height": "The height of the rectangle"},
    returns = "The area as a f64",
    example = "let area = calculate_area(5.0, 3.0);\nassert_eq!(area, 15.0);"
)]
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

## Key Benefits

✅ **Structured & Type-Safe** - Documentation fields are parsed at compile time  
✅ **IDE Support** - Auto-completion, syntax highlighting, and refactoring tools work  
✅ **Programmatic** - Generate docs from data, validate completeness, enforce standards  
✅ **Consistent** - Uniform formatting and structure across your entire codebase  
✅ **Maintainable** - Documentation is data, not prose scattered in comments  

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cutedogs = "0.0.5"
```

## Quick Start

```rust
use cutedogs::document;

#[document(summary = "Adds two numbers together")]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

This generates clean, formatted rustdoc output while keeping your documentation structured and maintainable.

## Documentation Fields

### Core Documentation
```rust
#[document(
    summary = "Processes user data and returns formatted output",
    params = {"name": "The user's full name", "age": "Age in years"},
    returns = "A formatted string containing user information"
)]
fn process_user(name: &str, age: u32) -> String {
    format!("{} is {} years old", name, age)
}
```

### Examples & Testing
```rust
#[document(
    summary = "Divides two numbers with error handling",
    example = "let result = safe_divide(10.0, 2.0);\nassert_eq!(result, Ok(5.0));"
)]
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("Division by zero".to_string()) } else { Ok(a / b) }
}
```

### Unimplemented Functions
Mark functions as not yet implemented with clear messaging:

```rust
#[document(unimplemented)]
fn future_feature() -> String {
    unimplemented!()
}

#[document(unimplemented = "Waiting for database integration")]
fn connect_db() -> Result<(), String> {
    unimplemented!()
}
```

### Advanced Documentation
```rust
#[document(
    summary = "Advanced data processing with comprehensive documentation",
    params = {"data": "Input vector to process"},
    returns = "Processed data with doubled values",
    example = "let result = process_data(vec![1, 2, 3]);",
    panics = "Panics if input vector is empty",
    safety = "Safe to use with any non-empty input",
    since = "1.0.0",
    see_also = "helper_function, related_processor",
    note = "This function is optimized for large datasets"
)]
fn process_data(data: Vec<i32>) -> Vec<i32> {
    assert!(!data.is_empty(), "Input cannot be empty");
    data.into_iter().map(|x| x * 2).collect()
}
```

### Deprecation Management
```rust
#[document(
    summary = "Legacy function for backwards compatibility",
    deprecated = "Use new_process_function() instead for better performance",
    deprecated_since = "2.0.0"
)]
fn old_process_function() -> String {
    "legacy".to_string()
}
```

## Complete Field Reference

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `summary` | String | Brief function description | `"Calculates rectangle area"` |
| `params` | Object | Parameter descriptions | `{"width": "Width in pixels", "height": "Height in pixels"}` |
| `returns` | String | Return value description | `"Area as f64 value"` |
| `example` | String | Code example with newlines | `"let x = func(1, 2);\nassert_eq!(x, 3);"` |
| `panics` | String | Panic conditions | `"Panics if divisor is zero"` |
| `safety` | String | Safety information | `"Safe for all valid inputs"` |
| `since` | String | Version introduced | `"1.0.0"` |
| `see_also` | String | Related functions (comma-separated) | `"related_func, helper_func"` |
| `note` | String | Important notes | `"Performance critical section"` |
| `deprecated` | String | Deprecation message | `"Use new_func() instead"` |
| `deprecated_since` | String | Version deprecated in | `"2.0.0"` |
| `unimplemented` | Flag/String | Mark as unimplemented | `unimplemented` or `"Feature pending"` |

## Generated Output

cutedogs generates clean, properly formatted rustdoc comments that integrate seamlessly with `cargo doc`. The structured approach ensures consistent formatting, proper markdown sections, and complete documentation coverage.

Your IDE treats the attribute syntax as first-class Rust code, providing:
- Syntax highlighting
- Auto-completion 
- Refactoring support
- Compile-time validation

## License

MIT OR Apache-2.0