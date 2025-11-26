use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => match &mut self.left {
                Some(left_node) => left_node.insert(value),
                None => {
                    self.left = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            },
            Ordering::Greater => match &mut self.right {
                Some(right_node) => right_node.insert(value),
                None => {
                    self.right = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            },
            Ordering::Equal => {} // Do nothing for duplicates
        }
    }
    fn remove(&mut self, value: T) {
        match self.value.cmp(&value) {
            Ordering::Equal => {
                let _ = drop(self);
            }
            _ => {
                self.remove(value);
            }
        }
    }

    fn len(&self, count: &mut i32) -> i32 {
        let mut left_count = 0;
        let mut right_count = 0;
        if self.left.is_some() {
            left_count = self.len(count)
        }
        if self.right.is_some() {
            right_count = self.len(count)
        }

        return 1 + left_count + right_count;
    }
}

#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    // new binary tree
    pub fn new() -> Self {
        Self { root: None }
    }
    // add a tree from the binary tree
    pub fn add(&mut self, value: T) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }))
            }
            Some(node) => {
                node.insert(value);
            }
        }
    }

    // remove a node from the binary tree
    pub fn remove(&mut self, value: T) {
        match &mut self.root {
            None => println!("Empty Tree"),
            Some(node) => {
                node.remove(value);
            }
        }
    }
    // get the length of the binary tree
    pub fn len() {}
}
