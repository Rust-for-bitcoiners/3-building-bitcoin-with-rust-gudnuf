#![allow(unused)]

/* This module will be taught in the class */

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: Option::None,
        }
    }
}

struct LinkedList<T> {
    head: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: Option::None }
    }

    fn push(&mut self, val: T) {
        let mut node = Node::new(val);

        match self.head.take() {
            Some(old_head) => {
                node.next = Some(Box::new(old_head));
            }
            None => {}
        }

        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next.map(|node| *node);
                Some(node.val)
            }
            None => None,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList { head: None };
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_linked_list_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop(), None);
    }
}
