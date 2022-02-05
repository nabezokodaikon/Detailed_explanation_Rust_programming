use std::rc::Rc;

#[derive(Debug)]
struct GraundStation {}

fn main() {
    let base = Rc::new(GraundStation {});

    println!("{:?}", base);
}
