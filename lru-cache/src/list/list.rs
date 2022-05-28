#[allow(dead_code)]

use std::collections::LinkedList;

pub mod list {
    use std::ops::Deref;

    #[derive(Clone, Debug)]
    pub struct Node<T> {
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

    pub struct List<T> {
        head: Option<Box<Node<T>>>,
        tail: Option<Box<Node<T>>>,
        length: u32
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self { head: None, tail: None, length: 0 }
        }

        pub fn head(self) -> Option<T> {
            match self.head {
                Some(boxed_node) => {
                    Some((*boxed_node).val)
                },
                None => None
            }
        }

        pub fn tail(self) -> Option<T> {
            match self.tail {
                Some(boxed_node) => {
                    Some((*boxed_node).val)
                },
                None => None
            }
        }

        pub fn len(self) -> u32 {
            self.length
        }
    }

    pub struct ListIterator<T> {
        list: List<T>
    }

    impl<T> IntoIterator for List<T> {
        type Item = T;

        type IntoIter = ListIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            ListIterator { list: self }
        }
    }

    impl<T> Iterator for ListIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            
        }
    }

}