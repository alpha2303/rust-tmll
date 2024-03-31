use std::mem;

/*
 * A Bad Singly-Linked Stack
 * Adding basic features:
 * - Push to top of stack
 * - Pop from top of stack
 * - Clear stack using Drop trait
 * - Basic Test method
 */

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
// In hindsight, this looks a lot like Rust's Option type.

/// Struct representing the Linked List that keeps track of the head pointer.
pub struct List {
    head: Link,
}

impl List {
    /// Create a new List
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// Inserts value at the top of the Linked List stack
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

    /// Remove the first value from the Linked List stack and return it
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

impl Drop for List {
    fn drop(&mut self) {
        // cur_link is used to get the Link entity, could be empty or a node box,
        // used for list traversal.
        let mut cur_link: Link = mem::replace(&mut self.head, Link::Empty);
        // while cur_link contains a valid node
        while let Link::More(mut node_box) = cur_link {
            cur_link = mem::replace(&mut node_box.next, Link::Empty);
            // current node is moved to local scope while cur_link is assigned the next node for traversal
            // the next pointer of current node is also replaced with null pointer to prevent unbound recursion during drop.
            // Current node in local scope get dropped at the end of the loop.
            // The node stored in the box also gets deallocated in this process.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    /// Testing basic list functionalities
    #[test]
    fn basics() {
        let mut list: List = List::new();

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