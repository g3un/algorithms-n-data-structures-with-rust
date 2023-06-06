#[cfg(test)]
mod tests {
    use doubly_linked_list as dll;

    #[test]
    fn test_push() {
        let mut dll = dll::DoublyLinkedList::<i64>::new();
        assert_eq!(dll.len(), 0);
        assert_eq!(dll.is_empty(), true);

        dll.push(1);
        assert_eq!(dll.len(), 1);
        assert_eq!(dll.is_empty(), false);

        dll.push(2);
        assert_eq!(dll.len(), 2);
        assert_eq!(dll.is_empty(), false);

        dll.push(3);
        assert_eq!(dll.len(), 3);
        assert_eq!(dll.is_empty(), false);
    }

    #[test]
    fn test_pop() {
        let mut dll = dll::DoublyLinkedList::<i64>::new();
        assert_eq!(dll.is_empty(), true);

        dll.push(1);
        dll.push(2);
        dll.push(3);
        assert_eq!(dll.len(), 3);
        assert_eq!(dll.is_empty(), false);

        let elem = dll.pop();
        assert_eq!(dll.len(), 2);
        assert_eq!(dll.is_empty(), false);
        assert_eq!(elem, Some(3));

        let elem = dll.pop();
        assert_eq!(dll.len(), 1);
        assert_eq!(dll.is_empty(), false);
        assert_eq!(elem, Some(2));

        let elem = dll.pop();
        assert_eq!(dll.len(), 0);
        assert_eq!(dll.is_empty(), true);
        assert_eq!(elem, Some(1));

        let elem = dll.pop();
        assert_eq!(dll.len(), 0);
        assert_eq!(dll.is_empty(), true);
        assert_eq!(elem, None);
    }
}
