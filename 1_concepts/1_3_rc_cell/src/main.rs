use crate::global_stack::GlobalStack;

fn main() {
    let stack = GlobalStack::new(1);
    stack.push(2);

    let reference = &stack;
    reference.push(3);

    let cloned_reference = stack.clone();
    cloned_reference.push(4);

    println!("Value (Pop) {}", stack.pop().unwrap());
    println!("Value (Pop) {}", cloned_reference.pop().unwrap());
    println!("Value (Pop) {}", reference.pop().unwrap());
    println!("Value (Pop) {}", cloned_reference.pop().unwrap());
    println!("Value None (Pop) {}", stack.pop().is_none());
}

mod global_stack {
    use std::{cell::RefCell, rc::Rc};

    pub struct GlobalStack<T> {
        inner: Rc<RefCell<Vec<T>>>,
    }

    impl<T> GlobalStack<T> {
        pub fn new(value: T) -> Self {
            GlobalStack {
                inner: Rc::new(RefCell::new(vec![value])),
            }
        }

        pub fn push(&self, value: T) {
            self.inner.borrow_mut().push(value);
        }

        pub fn pop(&self) -> Option<T> {
            self.inner.borrow_mut().pop()
        }
    }

    impl<T> Clone for GlobalStack<T> {
        fn clone(&self) -> Self {
            Self {
                inner: Rc::clone(&self.inner),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_stack_initialization() {
            let stack = GlobalStack::new(1);
            assert_eq!(stack.pop(), Some(1));
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_is_lifo() {
            let stack = GlobalStack::new(1);
            stack.push(2);
            stack.push(3);

            assert_eq!(stack.pop(), Some(3));
            assert_eq!(stack.pop(), Some(2));
            assert_eq!(stack.pop(), Some(1));
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_shared_references_can_mutate() {
            let stack = GlobalStack::new(1);
            let reference1 = &stack;
            let reference2 = &stack;

            reference1.push(2);
            reference2.push(3);

            assert_eq!(reference1.pop(), Some(3));
            assert_eq!(reference2.pop(), Some(2));
            assert_eq!(stack.pop(), Some(1))
        }

        #[test]
        fn test_clone() {
            let owner1 = GlobalStack::new(1);
            let owner2 = owner1.clone();

            owner2.push(2);
            assert_eq!(owner1.pop(), Some(2));
            assert_eq!(owner2.pop(), Some(1));
            assert_eq!(owner1.pop(), None);
        }
    }
}
