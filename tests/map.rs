use std::collections::{BTreeMap, HashMap};

use derive_build::Build;

#[derive(Default, Build, Eq, PartialEq, Debug)]
struct Request {
    #[required]
    url: String,

    path: Option<String>,

    query: HashMap<String, String>,

    query_b_tree: BTreeMap<u16, String>,
}

#[test]
fn test_builder() {
    let request = Request::new("example.com")
        .path("tester")
        .query("hello", "goodbye")
        .query("foo", "bar")
        .query_b_tree(123, "hello");

    let query = {
        let mut map = HashMap::new();
        map.insert("hello".to_string(), "goodbye".to_string());
        map.insert("foo".to_string(), "bar".to_string());
        map
    };

    let query_b_tree = {
        let mut map = BTreeMap::new();
        map.insert(123, "hello".to_string());
        map
    };

    let expected = Request {
        url: "example.com".to_string(),
        path: Some("tester".to_string()),
        query,
        query_b_tree,
    };

    assert_eq!(request, expected);
}
