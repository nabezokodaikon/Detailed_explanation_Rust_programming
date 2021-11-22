fn main() {
    let fruit = vec!["apple", "banana", "orange"];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, "Foo");
}
