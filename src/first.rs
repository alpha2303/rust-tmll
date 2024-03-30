use std::mem;

/// Struct representing nodes of Linked List
struct Node {
    elem: i32,
    next: Link,
}

/// Enum representing valid states of list nodes,
/// while taking advantage of null pointer optimization.
enum Link {
    /// Used to represent an empty node
    Empty,
    /// Used to hold a pointer to the next node in the List.
    // Can never be a null pointer as Node will expect an element value.
    More(Box<Node>),
}

/// Struct representing the Linked List that keeps track of the head pointer.
pub struct List {
    head: Link,
}

impl List {
    /// Create a new List
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// Inserts value at the beginning of the Linked List
    pub fn push(&mut self, elem: i32) {
        // self.head is left unallocated when value is moved to next.
        // This is not allowed.
        // Hence replacing self.head with Link::Empty using mem::replace
        // and passing original value to next
        let new_node: Box<Node> = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    /// Remove the first value from the Linked List and return it
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None, // If list is empty
            Link::More(node) => {
                // Move reference of next node into self.head
                self.head = node.next;
                // return value of node replaced from head
                Some(node.elem)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    /// Testing basic list functionalities
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check if empty list behaves right
        assert_eq!(list.pop(), None);

        // Insert elements
        list.push(1);
        list.push(2);
        list.push(3);

        // Check removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push additional data
        list.push(5);
        list.push(4);

        // Check removal
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));

        // Check pop till empty list
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}