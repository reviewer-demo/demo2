#![allow(dead_code)]

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn multiply(_a: u32, _b: u32) -> u32 {
    todo!("Implement")
}

#[test]
fn test_add() {
    assert_eq!(10, add(7, 3));
}