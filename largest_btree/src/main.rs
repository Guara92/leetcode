use std::cell::RefCell;
use std::cmp::max;
use std::mem::replace;
use std::rc::Rc;

#[test]
fn test_1() {
    assert_eq!(
        vec![1, 3, 9],
        Solution::largest_values(TreeNode::maybe_from_vec(vec![
            Some(1),
            Some(3),
            Some(2),
            Some(5),
            Some(3),
            None,
            Some(9),
        ]))
    );
}

#[test]
fn test_2() {
    assert_eq!(
        vec![1, 3],
        Solution::largest_values(TreeNode::maybe_from_vec(vec![Some(1), Some(3),]))
    );
}

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn maybe_from_vec(input: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        Self::build_tree(input.as_slice(), 0)
    }

    fn build_tree(input: &[Option<i32>], index: usize) -> Option<Rc<RefCell<Self>>> {
        let mut maybe_root = None;
        if index < input.len() {
            if let Some(node) = *input.get(index).unwrap() {
                let mut root = Self::new(node);
                root.left = Self::build_tree(input, 2 * index + 1);
                root.right = Self::build_tree(input, 2 * index + 2);
                maybe_root = Some(Rc::new(RefCell::new(root)));
            }
        }
        maybe_root
    }
}

impl Solution {
    pub fn largest_values(maybe_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out = vec![];
        Self::recursive_find(&mut out, maybe_root, 0);
        out
    }

    fn recursive_find(out: &mut Vec<i32>, maybe_root: Option<Rc<RefCell<TreeNode>>>, depth: usize) {
        if let Some(root) = maybe_root {
            if depth == out.len() {
                out.push(root.borrow().val);
            } else {
                let max = max(*out.get(depth).unwrap(), root.borrow().val);
                let _ = replace(&mut out[depth], max);
            }
            Self::recursive_find(out, root.borrow().left.clone(), depth + 1);
            Self::recursive_find(out, root.borrow().right.clone(), depth + 1);
        }
    }
}

fn main() {
    Solution::largest_values(TreeNode::maybe_from_vec(vec![
        Some(1),
        Some(3),
        Some(2),
        Some(5),
        Some(3),
        None,
        Some(9),
    ]));
}
