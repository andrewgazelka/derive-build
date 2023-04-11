use derive_build::Build;

#[derive(Default, Build, Eq, PartialEq, Debug)]
struct Request<'a> {
    #[required]
    url: &'a str,

    path: Option<&'a str>,

    messages: Vec<&'a str>,
}

#[test]
fn test_builder() {
    let request = Request::new("example.com")
        .path("tester")
        .message("hello")
        .message("goodbye");

    let expected = Request {
        url: "example.com",
        path: Some("tester"),
        messages: vec!["hello", "goodbye"],
    };

    assert_eq!(request, expected);

    let request = Request::new("example.com");
    assert_eq!(
        request,
        Request {
            url: "example.com",
            ..Default::default()
        }
    );

    let request = Request::new("example.com").path("tester");
    assert_eq!(
        request,
        Request {
            url: "example.com",
            path: Some("tester"),
            ..Default::default()
        }
    );

    let request = Request::new("example.com")
        .message("hello")
        .message("goodbye");

    assert_eq!(
        request,
        Request {
            url: "example.com",
            messages: vec!["hello", "goodbye"],
            ..Default::default()
        }
    );
}
