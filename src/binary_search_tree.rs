use std::{
    cmp::Ordering::{Equal, Greater, Less},
    ops::Deref,
};

pub struct BinarySearchTree<T>
where
    T: Ord,
{
    data: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            data: None,
            left: None,
            right: None,
        }
    }

    pub fn search(&self, data: &T) -> bool {
        match &self.data {
            Some(stored_data) => match data.cmp(stored_data) {
                Equal => true,
                Less => {
                    // data < stored_data
                    // search in the left
                    match &self.left {
                        Some(node) => node.search(data),
                        None => false,
                    }
                }
                Greater => {
                    // data > stored_data
                    // search in the right
                    match &self.right {
                        Some(node) => node.search(data),
                        None => false,
                    }
                }
            },
            None => false,
        }
    }

    pub fn insert(&mut self, data: T) {
        match &self.data {
            None => self.data = Some(data),
            Some(stored_data) => {
                let target_node = if data < *stored_data {
                    &mut self.left
                } else {
                    &mut self.right
                };

                match target_node {
                    Some(ref mut node) => {
                        node.insert(data);
                    }
                    None => {
                        let mut node = Self::new();

                        node.insert(data);

                        *target_node = Some(Box::new(node));
                    }
                }
            }
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.min(),
            None => self.data.as_ref(),
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.max(),
            None => self.data.as_ref(),
        }
    }

    /// Returns a new iterator which iterates over this tree in order
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIterator::new(self)
    }
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

struct BinarySearchTreeIterator<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIterator<'a, T>
where
    T: Ord,
{
    pub fn new(tree: &'a BinarySearchTree<T>) -> Self {
        let mut iter = Self { stack: vec![tree] };

        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIterator<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();

            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.data.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn prequel_tree() -> BinarySearchTree<u32> {
        let mut tree = BinarySearchTree::new();
        tree.insert(16);
        tree.insert(7);
        tree.insert(28);
        tree.insert(3);
        tree.insert(21);
        tree.insert(36);
        tree.insert(70);
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_tree();

        assert!(tree.search(&16));
        assert!(tree.search(&7));
        assert!(tree.search(&28));
        assert!(tree.search(&3));
        assert!(tree.search(&21));
        assert!(tree.search(&36));
        assert!(tree.search(&70));

        assert!(!tree.search(&1));
        assert!(!tree.search(&90));
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_tree();
        let mut iter = tree.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&16));
        assert_eq!(iter.next(), Some(&21));
        assert_eq!(iter.next(), Some(&28));
        assert_eq!(iter.next(), Some(&36));
        assert_eq!(iter.next(), Some(&70));
        assert_eq!(iter.next(), None);
    }
}
