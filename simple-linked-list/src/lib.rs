use std::iter::FromIterator;

pub struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        let mut node = &self.head;

        while node.is_some() {
            counter += 1;
            node = &(node.as_ref().unwrap().next)
        }

        counter
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(
            Node {
                element,
                next: self.head.take()
            }
        ));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None
        }

        let node = self.head.take().unwrap();
        self.head = node.next;
        Some(node.element)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_none() {
            return None
        }

        let node = self.head.as_ref().unwrap();
        Some(&node.element)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        while let Some(data) = self.pop() {
            list.push(data)
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for elem in iter {
            list.push(elem)
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = vec![];
        let mut r = self.rev();
        while let Some(data) = r.pop() {
            result.push(data)
        }
        result
    }
}
