#[allow(dead_code)]

use std::collections::LinkedList;

pub mod list {
    use std::ops::Deref;

    pub struct List<T> {
        head: Option<Box<Node<T>>>,
        tail: Option<Box<Node<T>>>,
        size: u32
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self { head: None, tail: None, size: 0 }
        }

        pub fn head(self) -> Option<Box<Node<T>>> {
            self.head
        }

        pub fn tail(self) -> Option<Box<Node<T>>> {
            self.tail
        }

    }


    #[derive(Clone, Debug)]
    struct Node<T> {
        val: T,
        prev: Option<Box<Node<T>>>,
        next: Option<Box<Node<T>>>
    }

    impl<T> Node<T> {
        fn new(el: T, prev: Option<Box<Node<T>>>, next: Option<Box<Node<T>>>) -> Self {
            Self { val: el, prev: prev, next: next }
        }

        fn next(self) -> Option<Box<Node<T>>> {
            self.next
        }

        fn prev(self) -> Option<Box<Node<T>>> {
            self.prev
        }
    }

    impl<T> Deref for Node<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.val
        }
    }

}