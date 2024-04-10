#![allow(dead_code)]

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

#[test]
fn test_add() {
    assert_eq!(10, add(7, 3));
}

#[test]
fn test_mult() {
    assert_eq!(15, multiply(5, 3));
}
