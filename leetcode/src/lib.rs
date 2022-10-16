pub mod list {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode {
                next: None,
                val
            }
        }

        pub fn create(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            Some(Box::new(ListNode { val, next }))
        }

        pub fn from_slice(slice: &[i32]) -> Option<Box<ListNode>> {
            if slice.is_empty() {
                None
            } else {
                Some(Box::new(ListNode { 
                    val: slice[0], 
                    next: ListNode::from_slice(&slice[1..]) }))
            }
        }

        pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
            ListNode::from_slice(vec.as_slice())
        }
    }
}

pub mod tree {
    use std::cell::RefCell;
    use std::rc::Rc;

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

        pub fn create(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
