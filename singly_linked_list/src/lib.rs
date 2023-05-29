struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self { elem, next: None }
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn push(&mut self, elem: T) {
        let mut cur = &mut self.head;

        match cur {
            Some(_) => {
                while let Some(node) = cur {
                    cur = &mut node.next;
                }
            }
            None => {}
        }

        *cur = Some(Box::new(Node::new(elem)));
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let mut cur = &mut self.head;

        match cur {
            Some(_) => {
                // Only head node
                if let Some(node) = cur {
                    if node.next.is_none() {
                        let elem = node.elem;
                        *cur = None;
                        return Some(elem);
                    }
                }

                while let Some(node) = cur {
                    let next = node.next.as_ref().unwrap();

                    if next.next.is_none() {
                        let elem = next.elem;
                        node.next = None;
                        // Drop next here?
                        return Some(elem);
                    }

                    cur = &mut node.next;
                }

                None
            }
            // Empty list
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        let mut c = 0;
        let mut cur = &self.head;

        while let Some(node) = cur {
            cur = &node.next;
            c += 1;
        }

        c
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
