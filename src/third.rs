/*
 * A Persistent Singly-Linked Stack
 * - Shared ownership of elements
 * (Box cannot handle this, we use reference counters instead)
 */

use std::rc::Rc;

type NodeRc<T> = Rc<Node<T>>;
/// Type representing valid states of list nodes.
type Link<T> = Option<NodeRc<T>>;

/// Struct representing the Linked List
/// that keeps track of the head pointer.
pub struct List<T> {
    head: Link<T>,
}

/// Struct representing nodes of Linked List
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {

    /// Create a new List
    pub fn new() -> Self {
        List { head: None }
    }

    /// Returns a List of shared references of 
    /// the calling List's nodes, with the new 
    /// element inserted at the head
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    /// Returns a reference of the element
    /// stored at the head of the List, 
    /// else `None` if the List is empty
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node: &NodeRc<T>| &node.elem)
    }

    /// Returns a List of shared references 
    /// to all nodes following the calling List's head
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node: &NodeRc<T>| node.next.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
}

