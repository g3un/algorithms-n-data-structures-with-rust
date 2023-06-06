# Doubly-Linked List

[*src/lib.rs*](src/lib.rs)

Implemented using `Rc` and `RefCell`.

```rust
struct Node<T> {
    elem: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

// %<

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}
```
