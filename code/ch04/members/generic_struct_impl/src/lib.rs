use std::cmp::{Eq, Ordering, PartialEq};
use std::fmt::{self, Display};

#[derive(Debug, Default, Eq, Ord)]
pub struct Container<T> {
    pub item: T,
}

impl<T> Container<T> {
    pub fn new(item: T) -> Self {
        Container { item }
    }
}

impl<T> Display for Container<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Container {{ item: {} }}", self.item)
    }
}

impl<T: PartialEq> PartialEq for Container<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: Ord> PartialOrd for Container<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.item.partial_cmp(&other.item)
    }
}

#[cfg(test)]
mod tests {
    use super::Container;
    use std::cmp::{Eq, Ordering, PartialEq};

    #[test]
    fn test_new_with_integer() {
        let container = Container::new(5);
        assert_eq!(container.item, 5);
    }

    #[test]
    fn test_new_with_string() {
        let container = Container::new("Hello, world!".to_string());
        assert_eq!(container.item, "Hello, world!".to_string());
    }

    #[test]
    fn test_new_with_boolean() {
        let container = Container::new(true);
        assert_eq!(container.item, true);
    }

    #[test]
    fn test_equality_with_integers() {
        let container1 = Container::new(5);
        let container2 = Container::new(5);
        assert_eq!(container1, container2);
        assert_ne!(container1, Container::new(10));
    }

    #[test]
    fn test_equality_with_strings() {
        let container1 = Container::new("hello".to_string());
        let container2 = Container::new("hello".to_string());
        assert_eq!(container1, container2);
        assert_ne!(container1, Container::new("world".to_string()));
    }

    #[test]
    fn test_comparison_with_integers() {
        let container1 = Container::new(5);
        let container2 = Container::new(10);
        let container3 = Container::new(5);

        assert!(container1 < container2); // Less than
        assert!(container2 > container1); // Greater than
        assert!(container1 <= container3); // Less than or equal
        assert!(container1 >= container3); // Greater than or equal
    }

    #[test]
    fn test_ordering_with_custom_structs() {
        #[derive(Debug, Default, Eq, Ord)]
        pub struct CustomStruct<T> {
            pub data: T,
        }

        impl<T: PartialEq> PartialEq for CustomStruct<T> {
            fn eq(&self, other: &Self) -> bool {
                self.data == other.data
            }
        }

        impl<T: Ord> PartialOrd for CustomStruct<T> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        let container1 = Container::new(CustomStruct { data: 1 });
        let container2 = Container::new(CustomStruct { data: 2 });

        assert!(container1 < container2); // Less than
        assert!(container2 > container1); // Greater than
    }
}
