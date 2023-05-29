# Singly-Linked list

[*src/lib.rs*](src/lib.rs)

Implemented using `Box`.

```rust
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

// %<

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
```
