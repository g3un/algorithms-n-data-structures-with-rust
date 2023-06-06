use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node<T> {
    elem: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            prev: None,
            next: None,
        }
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn len(&self) -> usize
    where
        T: Copy,
    {
        let mut c = 0;
        let mut cur: Option<Rc<RefCell<Node<T>>>>;

        match &self.head {
            Some(node) => {
                cur = Some(Rc::clone(node));
            }
            None => {
                cur = None;
            }
        }

        while let Some(node) = cur {
            let node = node.borrow();
            match &node.next {
                Some(next) => {
                    cur = Some(Rc::clone(next));
                }
                None => {
                    cur = None;
                }
            }

            c += 1;
        }

        c
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn push(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node::new(elem)));

        if self.is_empty() {
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(new_node);
        } else {
            let tail_node = Rc::clone(self.tail.as_ref().unwrap());
            new_node.borrow_mut().prev = Some(Rc::downgrade(&tail_node));
            tail_node.borrow_mut().next = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
        }
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        if self.is_empty() {
            None
        } else {
            let tail_node = Rc::clone(self.tail.as_ref().unwrap());
            let elem = tail_node.borrow().elem;

            let prev_node = &tail_node.borrow().prev;

            match prev_node {
                Some(prev_node) => {
                    let prev_node = prev_node.upgrade().unwrap();
                    prev_node.borrow_mut().next = None;
                    self.tail = Some(Rc::clone(&prev_node));
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }

            Some(elem)
        }
    }
}
