use std::borrow::Cow;

use derive_build::Build;

#[derive(Default, Build, Eq, PartialEq, Debug)]
struct Request<'a> {
    #[required]
    url: Cow<'a, str>,

    path: Option<Cow<'a, str>>,

    messages: Vec<Cow<'a, str>>,
}

#[test]
fn test_builder() {
    let request = Request::new("example.com")
        .path("tester")
        .message("hello")
        .message("goodbye");

    let expected = Request {
        url: "example.com".into(),
        path: Some("tester".into()),
        messages: vec!["hello".into(), "goodbye".into()],
    };

    assert_eq!(request, expected);

    let request = Request::new("example.com");
    assert_eq!(
        request,
        Request {
            url: "example.com".into(),
            ..Default::default()
        }
    );

    let request = Request::new("example.com").path("tester");
    assert_eq!(
        request,
        Request {
            url: "example.com".into(),
            path: Some("tester".into()),
            ..Default::default()
        }
    );

    let request = Request::new("example.com")
        .message("hello")
        .message("goodbye");

    assert_eq!(
        request,
        Request {
            url: "example.com".into(),
            messages: vec!["hello".into(), "goodbye".into()],
            ..Default::default()
        }
    );
}
