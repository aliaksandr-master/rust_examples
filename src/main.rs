mod enum_versus_bool;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_cell() {
    use std::cell::Cell;

    let a = Cell::new(123);

    assert_eq!(a.get(), 123);
    a.set(444);
    assert_eq!(a.get(), 444);
    assert_eq!(a.get(), 444);
}
