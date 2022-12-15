// https://leetcode.com/problems/remove-element/

// Given an integer array nums and an integer val,
// remove all occurrences of val in nums in-place.
// The relative order of the elements may be changed.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    // unknown if elements are sorted or not.
    nums.sort();

    while nums.contains(&val) {
        let pos = nums.iter().position(|x| *x == val).unwrap();
        nums.remove(pos);
        println!("nums: {:?}", nums);
    }

    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input: &mut Vec<i32> = &mut vec![3, 2, 2, 3];
        let result = remove_element(input, 3);
        assert_eq!(result, 2);
    }
    #[test]
    fn case_2() {
        let input: &mut Vec<i32> = &mut vec![0, 1, 2, 2, 3, 0, 4, 2];
        let result = remove_element(input, 2);
        assert_eq!(result, 5);
    }
}
