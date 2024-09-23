# Rust Constructor implement macro
## Usage
```rs
#[constructor::constructor]
#[derive(Debug, PartialEq, Eq, Clone)]
struct Test {
    a: String,
    b: i32,
}
```
