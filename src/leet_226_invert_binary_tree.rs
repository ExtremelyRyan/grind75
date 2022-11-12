// https://leetcode.com/problems/invert-binary-tree/
// Given the root of a binary tree, invert the tree, and return its root.
use crate::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {}
}
