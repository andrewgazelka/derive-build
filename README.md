# derive-build

[![Crates.io](https://img.shields.io/crates/v/derive-build.svg?style=plastic)](http://crates.io/crates/derive-build)

An opinionated builder macro in Rust!

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

## Features

- [x] Required fields
- [x] Optional fields
- [x] Default values for optional fields
- [ ] Default values for required fields (instead make option)
- [x] Support for `Option<T>` fields
- [x] Support for `Vec<T>` fields
- [x] Support for `Cow<'a, T>` fields
- [x] Support for references (`&'a T`) fields
- [x] Automatic `Into<T>` conversions
- [x] Automatic singularization of field names
- [ ] Support for `HashMap<K, V>` fields
- [ ] Support for `HashSet<T>` fields
- [ ] Support for `BTreeMap<K, V>` fields
- [ ] Support for `BTreeSet<T>` fields
- [ ] Support for `Box<T>` fields
- [ ] Support for `Rc<T>` fields
- [ ] Support for `Arc<T>` fields