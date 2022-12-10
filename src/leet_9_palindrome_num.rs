// Given an integer x, return true if x is a palindrome
// and false otherwise.

fn main() {}

pub fn is_palindrome(x: i32) -> bool {
    let s_rev: Vec<char> = x.to_string().chars().rev().collect();

    for (i, ch) in x.to_string().char_indices() {
        if ch != s_rev[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_palindrome(121), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_palindrome(-121), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(10), false);
    }
}
