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

mod mute_me_somehow {
    use std::{pin::Pin, rc::Rc};

    trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>) {}
    }

    impl MutMeSomehow for String {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            self.push_str("mutated");
        }
    }

    impl MutMeSomehow for &[u8] {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let sliced = self.get_mut();
            *sliced = &[0; 10];
        }
    }

    impl<T> MutMeSomehow for Vec<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {}
    }

    impl<T> MutMeSomehow for Rc<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {}
    }

    impl<T: Default> MutMeSomehow for Box<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let boxed = self.get_mut();
            **boxed = T::default();
        }
    }

    #[cfg(test)]
    mod test {
        use std::pin::Pin;

        use super::MutMeSomehow;

        #[test]
        fn mut_string() {
            let mut sut = String::new();
            Pin::new(&mut sut).mut_me_somehow();
            assert_eq!(sut, "mutated");
        }

        #[test]
        fn mut_slice_u8() {
            let mut sut: &[u8] = &[1; 10];
            Pin::new(&mut sut).mut_me_somehow();
            assert_eq!(sut, &[0; 10]);
        }
    }
}

mod future {
    use core::{future::Future, pin::Pin, todo};
    use pin_project::pin_project;

    #[pin_project]
    struct MeasurableFuture<Fut: Future> {
        #[pin]
        inner_future: Fut,
        started_at: Option<std::time::Instant>,
    }

    impl<Fut: Future> Future for MeasurableFuture<Fut> {
        type Output = Fut::Output;

        fn poll(
            self: core::pin::Pin<&mut Self>,
            cx: &mut core::task::Context<'_>,
        ) -> core::task::Poll<Self::Output> {
            let this = self.project();
            let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
            let _: &mut U = this.unpinned; // Normal reference to the field
            todo!()
        }
    }
}
