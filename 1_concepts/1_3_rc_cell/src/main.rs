use crate::global_stack::GlobalStack;

fn main() {
    let mut stack = GlobalStack::new(1);
    println!("First {}", stack.first().unwrap());
    let value = stack.pop();
    println!("Popped {}", value.unwrap());
    stack.push(1);
    println!("First {}", stack.first().unwrap());
}

mod global_stack {
    pub struct GlobalStack<T> {
        stack: Vec<T>,
    }

    impl<T> GlobalStack<T> {
        pub fn new(value: T) -> Self {
            GlobalStack { stack: vec![value] }
        }

        pub fn first(&self) -> Option<&T> {
            self.stack.first()
        }
        pub fn push(&mut self, value: T) {
            self.stack.push(value);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.stack.pop()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_stack_initialization() {
            let stack = GlobalStack::new(1);
            assert_eq!(stack.first(), Some(&1));
        }

        #[test]
        fn test_stack_pop_initial_value() {
            let mut stack = GlobalStack::new(1);
            let value = stack.pop();
            assert_eq!(value, Some(1));
        }

        #[test]
        fn test_stack_push_one_value() {
            let mut stack = GlobalStack::new(1);
            stack.push(2);
            assert_eq!(stack.pop(), Some(2));
            assert_eq!(stack.pop(), Some(1));
        }
    }
}
