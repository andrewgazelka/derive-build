# derive-build

[![Crates.io](https://img.shields.io/crates/v/derive-build.svg?style=plastic)](http://crates.io/crates/derive-build)

A powerful builder macro in Rust!

```rust
#[derive(Default, Build, Eq, PartialEq, Debug)]
struct Request {
    #[required]
    url: String,

    path: Option<String>,

    messages: Vec<String>,
}

fn main() {
    let request = Request::new("example.com")
        .path("tester")
        .message("hello")
        .message("goodbye");

    let expected = Request {
        url: "example.com".to_string(),
        path: Some("tester".to_string()),
        messages: vec!["hello".to_string(), "goodbye".to_string()],
    };

    assert_eq!(request, expected);
}

```