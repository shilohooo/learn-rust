use crate::common::setup;

pub mod common;

#[test]
fn test_add() {
    setup();
    assert_eq!(adder::add(1, 2), 3);
}
