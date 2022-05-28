#[allow(dead_code)]

use std::collections::LinkedList;

pub mod list {
    use std::ops::Deref;
    type ListNode<T> = Box<Node<T>>;

    #[derive(Clone, Debug)]
    pub struct Node<T> {
        val: T,
        prev: Option<ListNode<T>>,
        next: Option<ListNode<T>>,
    }

    impl<T> Node<T> {
        fn new(el: T, prev: Option<ListNode<T>>, next: Option<ListNode<T>>) -> Self {
            Self { val: el, prev: prev, next: next }
        }

        fn next(self) -> Option<ListNode<T>> {
            self.next
        }

        fn prev(self) -> Option<ListNode<T>> {
            self.prev
        }
        fn get(self) -> T {
            self.val
        }
    }

    impl<T> Deref for Node<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.val
        }
    }

    #[derive(Clone, Debug)]
    pub struct List<T> {
        head: Option<ListNode<T>>,
        tail: Option<ListNode<T>>,
        length: u32
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self { head: None, tail: None, length: 0 }
        }

        pub fn head(self) -> Option<ListNode<T>> {
            match self.head {
                Some(boxed_node) => {
                    Some(boxed_node)
                },
                None => None
            }
        }

        pub fn tail(self) -> Option<ListNode<T>> {
            match self.tail {
                Some(boxed_node) => {
                    Some(boxed_node)
                },
                None => None
            }
        }

        pub fn len(self) -> u32 {
            self.length
        }
    }

    pub struct ListIterator<T> {
        list: List<T>,
        cursor: Option<ListNode<T>>
    }

    impl<T> IntoIterator for List<T> {
        type Item = T;
        type IntoIter = ListIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            ListIterator { list: self, cursor: None }
        }
    }

    impl<T> Iterator for ListIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            todo!("Implement the next method :)");
        //     self.cursor = self.cursor.as_ref().unwrap().next();
        //     let current = self.cursor;
        //     match current {
        //         Some(boxed_node) => {
        //             match boxed_node.next() {
        //                 Some(next_node) => {
                            
        //                     Some(next_node.get())
        //                 },
        //                 None => None
        //             }
        //         },
        //         None => None
        //     }
        }
    }

}