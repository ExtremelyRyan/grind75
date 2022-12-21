// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer.
// The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
// Increment the large integer by one and return the resulting array of digits.

// check each number in reverse order and add 1, modulo 10
// check if mod 10 is 0
//  if true -> return
//  otherwise, continue the loop.
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut nums = digits.clone();

    for n in nums.iter_mut().rev() {
        *n = (*n + 1) % 10;

        match *n % 10 != 0 {
            true => {
                return nums;
            }
            _ => continue,
        }
    }
    [&vec![1], &nums[..]].concat()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let v = vec![1, 2, 3];
        assert_eq!(plus_one(v), vec![1, 2, 4]);
    }

    #[test]
    fn case2() {
        let v = vec![4, 3, 2, 1];
        assert_eq!(plus_one(v), vec![4, 3, 2, 2]);
    }

    #[test]
    fn case3() {
        let v = vec![9];
        assert_eq!(plus_one(v), vec![1, 0]);
    }
}
