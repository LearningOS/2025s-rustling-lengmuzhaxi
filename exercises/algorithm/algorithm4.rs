use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 在以当前节点为根的子树中插入值
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left_child) = self.left {
                    left_child.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right_child) = self.right {
                    right_child.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // 重复值，不做任何操作
            }
        }
    }

    // 在以当前节点为根的子树中查找值
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                if let Some(ref left_child) = self.left {
                    left_child.search(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref right_child) = self.right {
                    right_child.search(value)
                } else {
                    false
                }
            }
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(ref mut node) => {
                node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.search(value),
        }
    }
}

impl<T> Display for TreeNode<T>
where
    T: Ord + Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // 这里简单显示节点值，子节点不显示
        write!(f, "{}", self.value)
    }
}

impl<T> Display for BinarySearchTree<T>
where
    T: Ord + Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // 这里简单显示 root 节点
        match self.root {
            Some(ref node) => write!(f, "{}", node),
            None => write!(f, "Empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 初始不存在值 1
        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 仍然不存在 1 和 6
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        // 检查树根左右均为 None，表示重复插入没有生成新节点
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
