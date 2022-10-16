use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
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
      right: None
    }
  }
}

pub struct BSTIterator {
  stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {

  fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
    let mut iter = BSTIterator { stack: Vec::new() };

    iter.traverse_left(root);

    iter
  }

  fn traverse_left(&mut self, node: Option<Rc<RefCell<TreeNode>>>) {
    let mut opt_node = node.clone();

    while let Some(rc) = opt_node {
      self.stack.push(rc.clone());

      opt_node = (&*rc).borrow().left.clone();
    }
  }
  
  fn next(&mut self) -> i32 {
    match self.stack.pop() {
      Some(rc) => {
        self.traverse_left((&*rc).borrow().right.clone());

        (&*rc).borrow().val
      }

      None => panic!("Out of elements!")
    }
  }
  
  fn has_next(&self) -> bool {
      !self.stack.is_empty()
  }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {
    println!("Hello, world!");
}
