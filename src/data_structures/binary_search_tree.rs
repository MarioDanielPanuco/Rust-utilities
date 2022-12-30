#![allow(dead_code)]

use std::fmt::Display;

type TreeNode<T> = Option<Box<Node<T>>>;
type BST<T> = Node<T>;

#[derive(Debug)]
struct Node<T: Display> {
    left: TreeNode<T>,
    right: TreeNode<T>,
    value: T,
}

trait BinaryTree<T> {
    fn pre_order(&mut self);
    fn in_order(&mut self);
    fn post_order(&mut self);
}

trait BinarySearchTree<T: PartialOrd + Display>: BinaryTree<T> {
    fn insert(&mut self, value: T);
    fn remove(&mut self, value: T) -> bool;
    fn remove_node(&mut self);
    fn search(&mut self, key: T) -> T;
    fn find_min(&self) -> T;
    fn find_max(&self) -> T;
}

impl<T: Display> Node<T> {
    fn new(value: T) -> Self {
        Node {
            left: None,
            right: None,
            value,
        }
    }
}

impl<T> BinaryTree<T> for Node<T>
    where T: PartialOrd + Display {
    fn pre_order(&mut self) {
        println!("{}", self.value);
        if let Some(ref mut left) = self.left {
            left.pre_order();
        }
        if let Some(ref mut right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&mut self) {
        if let Some(ref mut left) = self.left {
            left.in_order();
        }

        println!("{}", self.value);

        if let Some(ref mut right) = self.right {
            right.in_order();
        }
    }

    fn post_order(&mut self) {
        if let Some(ref mut left) = self.left {
            left.post_order();
        }

        if let Some(ref mut right) = self.right {
            right.post_order();
        }
        println!("{}", self.value);
    }
}

impl<T> BinarySearchTree<T> for Node<T>
    where T: PartialOrd + Display + Copy {
    fn insert(&mut self, value: T) {
        if self.value < value {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        } else if let Some(ref mut left) = self.left {
            left.insert(value);
        } else {
            self.left = Some(Box::new(Node::new(value)));
        }
    }

    fn remove(&mut self, value: T) -> bool {
        if value <= self.value {
            if let Some(ref mut left) = self.left {
                left.remove(value);
            }
        } else if value > self.value {
            if let Some(ref mut right) = self.right {
                right.remove(value);
            }
        } else {
            self.remove_node();
        }
        todo!()
    }

    fn search(&mut self, key: T) -> T {
        todo!()
    }

    fn find_min(&self) -> T {
        if let Some(ref left) = self.left {
            left.find_min()
        } else {
            self.value
        }
    }

    fn find_max(&self) -> T {
        if let Some(ref right) = self.right {
            right.find_max()
        } else {
            self.value
        }
    }

    fn remove_node(&mut self) {
        if let Some(node) = self { 
            if node.right.is_some() {
                let mut sptr = &mut node.right as *mut self; 
                loop {
                    let = iu
                }
            }
        }
        todo!()
/*        if let Some(node) = self {
            if node.right.is_some() {
                let mut = sptr = &mut node.right as *mut Self;
                loop {
                    let successor = unsafe { &mut *sptr };
                }
            }
        }*/
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::binary_search_tree::*;
    use super::BST;

    fn create_bst() -> BST<i32> {
        let mut root = BST::<i32>::new(4);
        root.insert(2);
        root.insert(4);
        root.insert(5);
        root.insert(6);
        root.insert(1);
        root
    }

    #[test]
    fn insert() {
        let mut root = BST::<i32>::new(4);
        root.insert(2);
        root.insert(4);
        root.insert(5);
        root.insert(6);
        root.insert(1);

        assert_eq!(root.value, 4);

        if let Some(ref left) = root.left {
            assert_eq!(left.value, 2);

            if let Some(ref right) = left.right {
                assert_eq!(right.value, 4);
            }

            if let Some(ref left) = left.left {
                assert_eq!(left.value, 1)
            }
        }

        if let Some(ref right) = root.right {
            assert_eq!(right.value, 5);

            if let Some(ref right) = right.right {
                assert_eq!(right.value, 6);
            }
        }

        println!("Pre Order traversal");
        root.pre_order();
        println!("In Order traversal");
        root.in_order();
        println!("Post Order traversal");
        root.post_order();
    }

    #[test]
    fn test_remove() {
        let mut root = create_bst();

        println!("{}", root.remove(4));
    }

    #[test]
    fn test_max() {
        let mut root = create_bst();

        println!("\nNode's max: {}", root.find_max());

        assert_eq!(root.find_max(), 6);
    }
}

