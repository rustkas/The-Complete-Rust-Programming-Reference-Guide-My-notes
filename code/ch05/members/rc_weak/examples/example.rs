use rc_weak::LinkedList;

fn main() {
  let mut list: LinkedList<i32> = LinkedList::new();
  list = list.append(1);
  println!("{:?}", list);
}