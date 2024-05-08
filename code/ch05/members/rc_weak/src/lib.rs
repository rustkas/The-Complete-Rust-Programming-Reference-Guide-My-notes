use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

#[derive(Debug)]
pub struct LinkedListNode<T> {
    pub next: Option<Rc<RefCell<LinkedListNode<T>>>>,
    pub prev: Option<Weak<RefCell<LinkedListNode<T>>>>,
    pub data: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn append(&mut self, data: T) -> Self {
        let new_node = Rc::new(RefCell::new(LinkedListNode {
            data: data,
            next: self.head.clone(),
            prev: None,
        }));

        match self.head.clone() {
            Some(node) => {
                node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            }
            None => {}
        }

        LinkedList {
            head: Some(new_node),
        }
    }
}

impl<T: fmt::Display> fmt::Display for LinkedListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: PartialEq> PartialEq for LinkedListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: PartialOrd> PartialOrd for LinkedListNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut current = self.head.clone();
        while let Some(node) = current {
            write!(f, "{}", node.borrow().data)?;
            if node.borrow().next.is_some() {
                write!(f, ", ")?;
            }
            current = node.borrow().next.clone();
        }
        write!(f, "]")
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_display() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list = list.append(1).append(2).append(3);
        assert_eq!(format!("{}", list), "[3, 2, 1]");
    }

    #[test]
    fn test_linked_list_node_eq() {
        let node1 = Rc::new(RefCell::new(LinkedListNode {
            data: 1,
            next: None,
            prev: None,
        }));
        let node2 = Rc::new(RefCell::new(LinkedListNode {
            data: 1,
            next: None,
            prev: None,
        }));
        assert_eq!(node1, node2);
    }

    #[test]
    fn test_linked_list_node_partial_cmp() {
        let node1 = Rc::new(RefCell::new(LinkedListNode {
            data: 1,
            next: None,
            prev: None,
        }));
        let node2 = Rc::new(RefCell::new(LinkedListNode {
            data: 2,
            next: None,
            prev: None,
        }));
        assert_eq!(node1.partial_cmp(&node2), Some(std::cmp::Ordering::Less));
    }
}
