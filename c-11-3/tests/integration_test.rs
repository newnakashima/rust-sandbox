use c_11_3;

mod common;

#[test]
fn it_add_two() {
    common::setup();
    assert_eq!(4, c_11_3::add_two(2));
}
