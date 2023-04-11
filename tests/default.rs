use derive_build::Build;

#[derive(Build, Eq, PartialEq, Debug)]
struct Request {
    #[required]
    url: String,

    path: Option<String>,

    messages: Vec<String>,

    #[default = 123]
    protocol_version: u32,
}

#[test]
fn test_builder() {
    let request = Request::new("example.com")
        .path("tester")
        .message("hello")
        .message("goodbye");

    let expected = Request {
        url: "example.com".to_string(),
        path: Some("tester".to_string()),
        messages: vec!["hello".to_string(), "goodbye".to_string()],
        protocol_version: 123,
    };

    assert_eq!(request, expected);
}
