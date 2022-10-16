use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use leetcode::tree::TreeNode;

pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    fn recurse(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, nums: &mut HashSet<i32>) -> bool {
        if let Some(node) = node {
            let node = node.borrow();

            if nums.contains(&(k - node.val)) {
                true
            } else {
                nums.insert(node.val);

                recurse(&node.left, k, nums) || recurse(&node.right, k, nums)
            }
        } else {
            false
        }
    }

    recurse(&root, k, &mut HashSet::new())
}

fn main() {
    println!("Hello, world!");
}
