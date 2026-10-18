use std::{pin::Pin, rc::Rc};

use say_hi::SayHi;

fn main() {
    let count: &[u8] = &[8];
    Pin::new(&count).say_hi();

    let mut my_string = String::new();
    my_string.push_str("Ceci");
    Pin::new(&my_string).say_hi();

    let my_vec = vec![1, 2, 3];
    Pin::new(&my_vec).say_hi();

    let ref_count = Rc::new(1);
    Pin::new(&ref_count).say_hi();

    let my_box = Box::new(1);
    Pin::new(&my_box).say_hi();
}

mod say_hi {
    use std::fmt::Debug;
    use std::rc::Rc;
    use std::{fmt, pin::Pin};

    pub trait SayHi: fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    impl<T: Debug> SayHi for Box<T> {}
    impl<T: Debug> SayHi for Rc<T> {}
    impl<T: Debug> SayHi for Vec<T> {}
    impl SayHi for String {}
    impl SayHi for &[u8] {}
}
