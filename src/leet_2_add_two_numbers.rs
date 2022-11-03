use crate::ListNode;

/*
You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero,
except the number 0 itself.
 */

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
let mut l1_ref = l1.clone();
let mut l2_ref = l2.clone();

    let mut l1_vec: Vec<i32> = Vec::new();
    let mut l2_vec: Vec<i32> = Vec::new();

    for value in l1_ref {
        l1_vec.push(value.val);
        l1_ref = l1.as_ref().unwrap().next;
    }
    l2_ref.into_iter().for_each(|value| {
        l2_vec.push(value.val); 
    });
    println!("list1: {l1_vec:?}");
    println!("list2: {l2_vec:?}");

    None
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn ex1() {
        let list1 = ListNode::from_vec(&vec![2, 4, 3]); // 342
        let list2 = ListNode::from_vec(&vec![5, 6, 4]); // 465

        let mut result = add_two_numbers(list1, list2);

        let check = vec![7, 0, 8];

        let mut i = 0;
        while let Some(entry) = result {
            assert_eq!(entry.val, check[i]);
            result = entry.next;
            i += 1;
        }
        assert!(i == check.len());
    }
}
