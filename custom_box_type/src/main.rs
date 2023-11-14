use std::ops::Deref;

struct CustomBox<T>(T);

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0         //returns the reference to the first value in the tuple struct
    }
}

impl<T> CustomBox<T> {
    fn new(x: T) -> CustomBox<T> {
        CustomBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let m = CustomBox::new(String::from("Rust"));
    hello(&m);

    let x = 5;
    let y = CustomBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);      // rust ran *(y.deref())
}