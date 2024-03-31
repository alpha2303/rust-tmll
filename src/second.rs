/*
 * An Ok Singly-Linked Stack
 * - Converted the Link enum into an Option type
 * - Replaced mem::replace and match with in-built Option methods
 * - Introduced Generic Type
 * - Peek and mutable peek 
 * - Convert into iterator using IntoIter, Iter, IterMut
 */

// Converting Link into an Option type instead of an enum,
// because why re-invent the wheel?
type NodeBox<T> = Box<Node<T>>;
/// Type representing valid states of list nodes. 
type Link<T> = Option<NodeBox<T>>;

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

/// Iterator providing sequential access 
/// to element values while consuming List
pub struct IntoIter<T>(List<T>);

/// Iterator providing sequential access to 
/// references of list nodes, if available
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// Iterator providing sequential access to 
/// mutable references of list nodes in sequence, if available
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    /// Create a new List
    pub fn new() -> Self {
        List { head: None }
    }

    /// Inserts value at the top of the Linked List stack
    pub fn push(&mut self, elem: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            elem: elem,
            next: self.head.take(), 
            // self.head.take() is same as 
            // mem::replace(&mut self.head, None)
        });

        self.head = Some(new_node);
    }

    /// Remove the first value from the Linked List stack and return it
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node: NodeBox<T>| {
            self.head = node.next;
            node.elem
        })
        // Same as below. Options are really convenient, aren't they?
        //
        // match self.head.take() {
        //     None => None, // If list is empty
        //     Some(node) => {
        //         // Move reference of next node into self.head
        //         self.head = node.next;
        //         // return value of node replaced from head
        //         Some(node.elem)
        //     },
        // }
    }

    /// Retrieve a reference to the topmost element of the stack
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node: &NodeBox<T>| {
            &node.elem
        })
    }

    /// Retrieve a mutable reference to the topmost element of the stack
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node: &mut NodeBox<T>| {
            &mut node.elem
        })
    }

    /// Create an IntoIter struct from List
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// Create an Iter struct from List
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
        // as_deref() is equivalent to 
        // map(|node: &NodeBox<T>| &**node)
    }

    /// Create an IterMut struct from List
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // cur_link is used for list traversal
        let mut cur_link: Link<T> = self.head.take();
        // while cur_link contains a valid node
        while let Some(mut node_box) = cur_link {
            cur_link = node_box.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    
    type Item = T;
    
    /// Get the next value of the iterator
    fn next(&mut self) -> Option<Self::Item> {
        // pop returns the node value instead of a reference
        self.0.pop()
    }
}

// Lifetime required here since Iter contains lifetime requirement
impl<'a, T> Iterator for Iter<'a, T> {
    
    type Item = &'a T;
    
    /// Get next element reference in iterator
    fn next(&mut self) -> Option<Self::Item> {
        // Item type handles lifetime requirement, 
        // so not required to specify here
        self.next.map(|node: &Node<T>| {
            self.next = node.next.as_deref();
            // as_deref() really makes getting 
            // references of Box values convenient
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    /// Get next mutable element reference in iterator
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node: &mut Node<T>| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::{List, IntoIter, Iter, IterMut};

    /// Testing basic list functionalities
    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();

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

    /// Test peek functionality of List
    #[test]
    fn peek() {
        let mut list: List<i32> = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // The answer to Life, the Universe and Everything.
        list.peek_mut().map(|value: &mut i32| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    /// Testing IntoIter struct functionality
    #[test]
    fn into_iter() {
        let mut list: List<i32> = List::new();
        
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter: IntoIter<i32> = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    /// Testing Iter struct functionality
    #[test]
    fn iter() {
        let mut list: List<i32> = List::new();
        
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter: Iter<i32> = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    /// Testing IterMut struct functionality
    #[test]
    fn iter_mut() {
        let mut list: List<i32> = List::new();
        
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter: IterMut<i32> = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}