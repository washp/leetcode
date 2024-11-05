#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }
    fn new_from_vec(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut current: Option<Box<ListNode>> = None;
        for val in vals.iter().rev() {
            if current.is_some() {
                let new_node = Some(Box::new(ListNode::new(*val, current)));
                current = new_node;
            } else {
                current = Some(Box::new(ListNode::new(*val, None)));
            }
        }
        current
    }
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut result: Option<Box<ListNode>> = None;
    let mut current = &mut result;
    while l1.is_some() || l2.is_some() || carry > 0 {
        let sum =
            l1.as_ref().map_or(0, |val| val.val) + l2.as_ref().map_or(0, |val| val.val) + carry;
        carry = sum / 10;
        let new_node = current.insert(Box::new(ListNode::new(sum % 10, None)));
        current = &mut new_node.next;
        l1 = l1.and_then(|node| node.next);
        l2 = l2.and_then(|node| node.next);
    }
    result
}

pub fn main() {
    println!("Not implemented!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = ListNode::new_from_vec(vec![2, 4, 3]);
        let l2 = ListNode::new_from_vec(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(result, ListNode::new_from_vec(vec![7, 0, 8]));
    }
    #[test]
    fn test2() {
        let l1 = ListNode::new_from_vec(vec![0]);
        let l2 = ListNode::new_from_vec(vec![0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(result, ListNode::new_from_vec(vec![0]));
    }
    #[test]
    fn test3() {
        let l1 = ListNode::new_from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::new_from_vec(vec![9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(result, ListNode::new_from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]));
    }
}
