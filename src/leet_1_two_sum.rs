use std::{collections::HashMap, vec};

/*
Given an array of integers nums and an integer target, return indices
of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution,
and you may not use the same element twice.

You can return the answer in any order.
*/
/// simple version
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
    for (i , i_val) in nums.iter().enumerate() {
        for j in i+1..nums.len()   {
            if i_val + nums[j] == target {
                return vec![i as i32,j as i32];
            } 
        }
    }
    vec![]
}

/// one pass hash table solution
pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hmap: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let compliment = target - nums[i];
        if hmap.contains_key(&compliment) {
            return vec![hmap[&compliment], i as i32];
        }
        hmap.insert(nums[i], i as i32);
    }

    vec![]
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn example1() {
        assert_eq!(two_sum_hashmap(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum2(vec![2, 5, 5, 11], 10), vec![1, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(two_sum_hashmap(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum2(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(two_sum_hashmap(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum2(vec![3, 3], 6), vec![0, 1]);
    }
}
