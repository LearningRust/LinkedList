use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None}
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node{
            data: data,
            next: mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.data
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter{ next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{ next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        for i in 0..10 {
            list.push(i);
        }

        for i in 9..1 {
            assert_eq!(list.pop(), Some(i));
        }

        list.push(70);
        list.push(71);

        assert_eq!(list.pop(), Some(71));
        assert_eq!(list.pop(), Some(70));
    }

    #[test]
    fn peek() {
         let mut list = List::new();
            assert_eq!(list.peek(), None);
            assert_eq!(list.peek_mut(), None);
            list.push(1); list.push(2); list.push(3);

            assert_eq!(list.peek(), Some(&3));
            assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }
}
