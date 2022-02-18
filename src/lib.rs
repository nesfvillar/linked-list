use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&self, value: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Option<Self> {
        self.head.clone().map(|node| Self {
            head: node.next.clone(),
        })
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.head.clone().map(|node| node.value.clone());
        *self = self.tail().unwrap_or_default();
        result
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut list = Self::new();

        for i in iter {
            list = list.prepend(i);
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new() {
        let list: List<u32> = List::new();
        assert_eq!(list.head, None);
    }

    #[test]
    fn test_prepend() {
        let list: List<u32> = List::new();
        let new_list = list.prepend(0);
        let new_new_list = new_list.prepend(1);
        assert_eq!(new_list.head.as_ref().unwrap().value, 0);
        assert_eq!(new_new_list.head.as_ref().unwrap().value, 1);
    }

    #[test]
    fn test_tail() {
        let list: List<u32> = List::new();
        let new_list = list.prepend(1);
        let new_new_list = new_list.prepend(0);
        assert_eq!(list.tail(), None);
        assert_eq!(new_list.tail(), Some(list));
        assert_eq!(new_new_list.tail(), Some(new_list));
    }

    #[test]
    fn test_into_iterator() {
        let mut list: List<u32> = List::new();
        list = list.prepend(1);
        list = list.prepend(0);
        for (i, value) in list.enumerate() {
            assert_eq!(value, i as u32);
        }
    }

    #[test]
    fn test_from_iterator() {
        let list = (0..10).rev().collect::<List<_>>();
        for (i, value) in list.enumerate() {
            assert_eq!(value, i)
        }
    }
}
