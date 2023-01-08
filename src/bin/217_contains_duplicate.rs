use std::collections::HashMap;

fn main() {
    let nums = [1,2,3,4].to_vec();
    assert_eq!(contains_duplicate(nums), false);
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut h: HashMap<i32, i32> = HashMap::new();
    let mut v: Vec<i32> = Vec::new();

    for (i,n) in nums.iter().enumerate() {
        if v.contains(&n) { return true; }
        v.insert(i,*n);
    }
 
    
    // for n in nums {
    //     if h.contains_key(&n) {
    //         return true;
    //     }
    //     h.insert(n, n);
    // }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let nums = [1,2,3,1].to_vec();
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn example2() {
        let nums = [1,2,3,4].to_vec();
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn example3() {
        let nums = [1,1,1,3,3,4,3,2,4,2].to_vec();
        assert_eq!(contains_duplicate(nums), true);
    }


}
