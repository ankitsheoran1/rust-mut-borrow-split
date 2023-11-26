use std::mem;


fn main() {

}

pub struct IterMutSlice<'a, T: 'a>(&'a mut[T]);

impl<'a, T> Iterator for IterMutSlice<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = mem::take(&mut self.0);
        if slice.is_empty() { return None; }

        let (l, r) = slice.split_at_mut(1);
        self.0 = r;
        l.get_mut(0)
    }
}

impl<'a, T> DoubleEndedIterator for IterMutSlice<'a, T> {
   

    fn next_back(&mut self) -> Option<Self::Item> {
        let slice = mem::take(&mut self.0);
        if slice.is_empty() { return None; }

        let len = self.0.len();
        let (l, r) = slice.split_at_mut(len - 1);
        self.0 = l;
        r.get_mut(0)
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct  Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
   head: Link<T>,
}

pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

impl<T> LinkedList<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_mut().map(|node| &mut **node))
    }
    // 2 - 1
    // head = 2 
    // 3 - 2
    // head = 3
    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self)-> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })

    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        // Test case for LinkedList
        let mut list = LinkedList { head: None };
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
