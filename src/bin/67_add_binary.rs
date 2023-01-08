// https://leetcode.com/problems/add-binary/

fn main() {}

//Given two binary strings a and b, return their sum as a binary string.

pub fn add_binary(_a: String, _b: String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = add_binary("11".to_string(), "1".to_string());
        assert_eq!(result, "100");
    }

    #[test]
    fn case_2() {
        let result = add_binary("1010".to_string(), "1011".to_string());
        assert_eq!(result, "10101");
    }
}
