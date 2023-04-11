# derive-build

A powerful builder macro in Rust!

```rust
#[derive(Default, Build, Eq, PartialEq, Debug)]
struct Request {
    #[required]
    url: String,

    path: Option<String>,

    messages: Vec<String>,
}
```