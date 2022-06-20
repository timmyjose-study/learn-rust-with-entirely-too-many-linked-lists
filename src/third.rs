//! A Persistent Singly-Linked Stack.

use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        let new_node = Rc::new(Node {
            elem,
            next: self.head.clone(),
        });
        List {
            head: Some(new_node),
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
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
            if let Ok(mut node) = Rc::try_unwrap(node) {
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
    fn test_persistent_list() {
        let list1 = List::new();
        assert!(list1.head().is_none());

        let list1 = list1.prepend(1);
        assert_eq!(list1.head(), Some(&1));

        let list1 = list1.prepend(2);
        let list1 = list1.prepend(3);
        let list1 = list1.prepend(4);
        let list1 = list1.prepend(5);

        let head1 = list1.head();
        assert_eq!(head1, Some(&5));

        let list2 = list1.tail();
        let head2 = list2.head();
        assert_eq!(head2, Some(&4));

        let list3 = list1.prepend(100);
        let head3 = list3.head();
        assert_eq!(head3, Some(&100));

        let mut iter3 = list3.iter();
        assert_eq!(iter3.next(), Some(&100));
        assert_eq!(iter3.next(), Some(&5));
        assert_eq!(iter3.next(), Some(&4));
        assert_eq!(iter3.next(), Some(&3));
        assert_eq!(iter3.next(), Some(&2));
        assert_eq!(iter3.next(), Some(&1));
        assert_eq!(iter3.next(), None);
    }
}
