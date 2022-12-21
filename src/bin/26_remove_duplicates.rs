pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input: &mut Vec<i32> = &mut vec![1, 1, 2];
        let result = remove_duplicates(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_2() {
        let input: &mut Vec<i32> = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = remove_duplicates(input);
        assert_eq!(result, 5);
    }
}
