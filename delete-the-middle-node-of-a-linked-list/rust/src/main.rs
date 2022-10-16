use leetcode::list::ListNode;

pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn list_length(node: &Option<Box<ListNode>>) -> usize {
        if let Some(node) = node {
            1 + list_length(&node.next)
        } else {
            0
        }
    }

    fn recurse(node: Option<Box<ListNode>>, pos: usize) -> Option<Box<ListNode>> {
        node.and_then(|mut node| {
            if pos > 0 {
                node.next = recurse(node.next, pos - 1);
                Some(node)
            } else {
                node.next
            }
        })
    }
    
    let half = list_length(&head) / 2;

    recurse(head.clone(), half)
}

fn main() {
    println!("{:?}", delete_middle(ListNode::from_slice(&[1,3,4,7,1,2,6])));
    println!("{:?}", delete_middle(ListNode::from_slice(&[1,2,3,4])));
}
