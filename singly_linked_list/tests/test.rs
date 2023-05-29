#[cfg(test)]
mod tests {
    use singly_linked_list as sll;

    #[test]
    fn test_push() {
        let mut sll = sll::SinglyLinkedList::<i64>::new();
        assert_eq!(sll.len(), 0);
        assert_eq!(sll.is_empty(), true);

        sll.push(1);
        assert_eq!(sll.len(), 1);
        assert_eq!(sll.is_empty(), false);

        sll.push(2);
        assert_eq!(sll.len(), 2);
        assert_eq!(sll.is_empty(), false);

        sll.push(3);
        assert_eq!(sll.len(), 3);
        assert_eq!(sll.is_empty(), false);
    }

    #[test]
    fn test_pop() {
        let mut sll = sll::SinglyLinkedList::<i64>::new();
        assert_eq!(sll.is_empty(), true);

        sll.push(1);
        sll.push(2);
        sll.push(3);
        assert_eq!(sll.len(), 3);
        assert_eq!(sll.is_empty(), false);

        let elem = sll.pop();
        assert_eq!(sll.len(), 2);
        assert_eq!(sll.is_empty(), false);
        assert_eq!(elem, Some(3));

        let elem = sll.pop();
        assert_eq!(sll.len(), 1);
        assert_eq!(sll.is_empty(), false);
        assert_eq!(elem, Some(2));

        let elem = sll.pop();
        assert_eq!(sll.len(), 0);
        assert_eq!(sll.is_empty(), true);
        assert_eq!(elem, Some(1));

        let elem = sll.pop();
        assert_eq!(sll.len(), 0);
        assert_eq!(sll.is_empty(), true);
        assert_eq!(elem, None);
    }
}
