/*
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.
*/

pub fn is_palindrome(s: String) -> bool {
    // convert to lowercase
    // remove all non-alphanumeric
    let forward: String = s
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect();

    // reverse string and compare
    let backward: String = forward.chars().rev().collect();

    forward == backward
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn example2() {
        assert!(!is_palindrome("race a car".to_string()));
    }

    #[test]
    fn example3() {
        assert!(is_palindrome(" ".to_string()));
    }
}
