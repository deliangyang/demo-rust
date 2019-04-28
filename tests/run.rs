extern crate helloworld;

use helloworld::add;

#[test]
fn test_add() {
    assert_eq!(4, add(2, 2))
}