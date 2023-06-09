use derive_build::Build;

#[derive(Default, Build, Eq, PartialEq, Debug)]
struct Request {
    #[required]
    url: String,

    path: Option<String>,

    messages: Vec<String>,
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
    };

    assert_eq!(request, expected);

    let request = Request::new("example.com");
    assert_eq!(
        request,
        Request {
            url: "example.com".to_string(),
            ..Default::default()
        }
    );

    let request = Request::new("example.com").path("tester");
    assert_eq!(
        request,
        Request {
            url: "example.com".to_string(),
            path: Some("tester".to_string()),
            ..Default::default()
        }
    );

    let request = Request::new("example.com")
        .message("hello")
        .message("goodbye");

    assert_eq!(
        request,
        Request {
            url: "example.com".to_string(),
            messages: vec!["hello".to_string(), "goodbye".to_string()],
            ..Default::default()
        }
    );
}
