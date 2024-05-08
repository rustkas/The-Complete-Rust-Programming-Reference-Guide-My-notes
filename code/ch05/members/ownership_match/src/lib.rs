use std::fmt;

#[derive(Debug)]
pub enum Food {
    Cake,
    Pizza,
    Salad
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Food::Cake, Food::Cake) => true,
            (Food::Pizza, Food::Pizza) => true,
            (Food::Salad, Food::Salad) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Food::Cake, Food::Cake) => Some(std::cmp::Ordering::Equal),
            (Food::Cake, _) => Some(std::cmp::Ordering::Greater),
            (_, Food::Cake) => Some(std::cmp::Ordering::Less),
            (Food::Pizza, Food::Pizza) => Some(std::cmp::Ordering::Equal),
            (Food::Pizza, _) => Some(std::cmp::Ordering::Greater),
            (_, Food::Pizza) => Some(std::cmp::Ordering::Less),
            (Food::Salad, Food::Salad) => Some(std::cmp::Ordering::Equal),
        }
    }
}

#[derive(Debug)]
pub struct Bag {
    food: Food
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.food)
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.food == other.food
    }
}

impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.food.partial_cmp(&other.food)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owntership_match_01() {
        let bag = Bag { food: Food::Cake };
        match &bag.food {
            Food::Cake => println!("I got cake"),
            a => println!("I got {:?}", a)
        }
        
        println!("{:?}", bag);
    }

    #[test]
    fn test_bag_display() {
        let bag = Bag { food: Food::Cake };
        assert_eq!(format!("{}", bag), "Cake");
    }

    #[test]
    fn test_bag_equality() {
        let bag1 = Bag { food: Food::Pizza };
        let bag2 = Bag { food: Food::Pizza };
        assert_eq!(bag1, bag2);
    }

    #[test]
    fn test_bag_ordering() {
        let bag1 = Bag { food: Food::Salad };
        let bag2 = Bag { food: Food::Pizza };
        let bag3 = Bag { food: Food::Salad };
        assert!(bag1 < bag2);
        assert_ne!(bag1, bag2);
        assert!(!(bag1 > bag2));
        assert_eq!(bag1, bag3);
    }
}
