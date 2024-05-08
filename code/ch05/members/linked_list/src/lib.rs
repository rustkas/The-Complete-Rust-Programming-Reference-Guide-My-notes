use std::fmt;
use std::rc::Rc; 

#[derive(Debug)] 
pub struct LinkedList<T> { 
    pub head: Option<Rc<Node<T>>> 
} 

#[derive(Debug)] 
pub struct Node<T> { 
    pub next: Option<Rc<Node<T>>>, 
    pub data: T 
} 

impl<T> LinkedList<T> { 
    pub fn new() -> Self { 
        LinkedList { head: None } 
    } 

    pub fn append(&self, data: T) -> Self { 
        LinkedList { 
            head: Some(Rc::new(Node { 
                data: data, 
                next: self.head.clone() 
            })) 
        } 
    } 
} 

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.next == other.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

 

    #[test]
    fn test_linked_list_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.head, None);
    }

    #[test]
    fn test_linked_list_append() {
        let list = LinkedList::new().append(1).append(2).append(3);
        assert_eq!(list.head.is_some(), true);
        let node = list.head.as_ref().unwrap();
        assert_eq!(node.data, 3);
        let next_node = node.next.as_ref().unwrap();
        assert_eq!(next_node.data, 2);
        let next_next_node = next_node.next.as_ref().unwrap();
        assert_eq!(next_next_node.data, 1);
        assert_eq!(next_next_node.next, None);
    }
}

