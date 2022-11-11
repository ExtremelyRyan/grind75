use crate::ListNode;
// NOT DONE
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
    let mut l1_ref = l1;
    let mut l2_ref = l2;

    let mut l1_vec: Vec<i32> = Vec::new();
    let mut l2_vec: Vec<i32> = Vec::new();

    let  _num1 = 0;
    let  _num2 = 0;

    let _sum = 0;

    while l1_ref.is_some() {
        l1_vec.push(l1_ref.as_ref().unwrap().val);
        l1_ref = l1_ref.unwrap().next;
    }
    while l2_ref.is_some() {
        l2_vec.push(l2_ref.as_ref().unwrap().val);
        l2_ref = l2_ref.unwrap().next;
    }

    //debugging
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
        // assert!(i == check.len());
    }
}
