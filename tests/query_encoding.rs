use godaddy_rust::ApiClient;

#[test]
fn build_query_pairs_repeats_keys_for_arrays() {
    let pairs = ApiClient::build_query_pairs(vec![("items", Some(vec!["a", "b"].into()))]);
    assert_eq!(
        pairs,
        vec![
            (String::from("items"), String::from("a")),
            (String::from("items"), String::from("b"))
        ]
    );
}
