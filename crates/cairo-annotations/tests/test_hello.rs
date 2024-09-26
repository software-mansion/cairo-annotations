use cairo_annotations::hello;

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, world!");
}
