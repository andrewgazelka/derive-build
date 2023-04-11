use std::collections::{BTreeSet, HashSet};

use derive_build::Build;

#[derive(Default, Build, Eq, PartialEq, Debug)]
struct Request {
    #[required]
    url: String,

    path: Option<String>,

    messages: HashSet<String>,

    b_tree_messages: BTreeSet<String>,
}

#[test]
fn test_builder() {
    let request = Request::new("example.com")
        .path("tester")
        .message("hello")
        .b_tree_message("goodbye");

    let expected = Request {
        url: "example.com".to_string(),
        path: Some("tester".to_string()),
        messages: ["hello".to_string()].into_iter().collect(),
        b_tree_messages: ["goodbye".to_string()].into_iter().collect(),
    };

    assert_eq!(request, expected);
}
