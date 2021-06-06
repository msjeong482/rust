use test_type;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_type::add_two(2));
}
