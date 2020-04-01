// use std::rc::Rc;
//
// pub struct List<T> {
//     head: Link<T>
// }
//
// type Link<T> = Option<Rc<Node<T>>>;
//
// struct Node<T> {
//     val: T,
//     next: Link<T>
// }
//
// impl<T> List<T> {
//     pub fn new() -> Self {
//         List { head: None }
//     }
//
//     pub fn append(&self, val: T) -> List<T> {
//         List {
//             head: Some(Rc::new(Node{
//                 val: val,
//                 next: self.head.clone()
//             }))
//         }
//     }
//
//     pub fn tail(&self) -> List<T> {
//         List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
//     }
//
//     pub fn head(&self) -> Option<&T> {
//         self.head.as_ref().map(|node| &node.val)
//     }
//
//     pub fn iter(&self) -> Iter<T> {
//         Iter { next: self.head.as_ref().map(|node| &node.val) }
//     }
// }
//
// pub struct Iter<'a, T> {
//     next: Option<&'a T>
// }
//
// impl<T> Iterator for Iter<T> {
//     type Item = &T;
//
//     fn next(&self) -> Option<Self::Item> {
//         self.next = self.next
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::List;
//
//     #[test]
//     fn basics() {
//         let list = List::new();
//         assert_eq!(list.head(), None);
//
//         let list = list.append(9).append(6).append(3);
//         assert_eq!(list.head(), Some(&3));
//
//         let list = list.tail();
//         assert_eq!(list.head(), Some(&6));
//
//         let list = list.tail();
//         assert_eq!(list.head(), Some(&9));
//
//         let list = list.tail();
//         assert_eq!(list.head(), None);
//
//         // Make sure tail on empty list works
//         let list = list.tail();
//         assert_eq!(list.head(), None);
//     }
// }
