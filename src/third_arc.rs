//! A Persistent and threadsafe Singly-Linked Stack

use std::sync::Arc;

type Link<T> = Option<Arc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        let new_node = Arc::new(Node {
            elem,
            next: self.head.clone(),
        });
        List {
            head: Some(new_node),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                cur_link = node.next.take();
            } else {
                break;
            }
        }
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_persistent_list_arc() {
        let list1 = List::new()
            .prepend(1)
            .prepend(2)
            .prepend(3)
            .prepend(4)
            .prepend(5);
        assert_eq!(list1.head(), Some(&5));

        let list2 = list1.tail();
        assert_eq!(list2.head(), Some(&4));

        let list3 = list2.prepend(100);
        assert_eq!(list3.head(), Some(&100));

        let mut iter = list3.iter();
        assert_eq!(iter.next(), Some(&100));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(list3.head(), Some(&100));
    }
}
