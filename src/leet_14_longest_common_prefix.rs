//Write a function to find the longest common prefix string amongst an array of strings.
//If there is no common prefix, return an empty string "".

// this is going to assume there are no more than 3 indexes in strs
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    if strs.len() == 1 {
        return strs.get(0).unwrap().to_string();
    }

    let first = strs.get(0).unwrap();

    for i in (0..first.len()).rev() {
        let prefix = &first[0..=i];
        dbg!(prefix);
        if strs.iter().all(|s| s.starts_with(prefix)) {
            return prefix.to_string();
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let result = longest_common_prefix(input);
        assert_eq!(result, "fl".to_string());
    }

    #[test]
    fn case2() {
        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let result = longest_common_prefix(input);
        assert_eq!(result, "".to_string());
    }
    #[test]
    fn case3() {
        let input = vec!["a".to_string()];
        let result = longest_common_prefix(input);
        assert_eq!(result, "a".to_string());
    }
    #[test]
    fn case4() {
        let input = vec![
            "flower".to_string(),
            "flower".to_string(),
            "flower".to_string(),
            "flower".to_string(),
        ];
        let result = longest_common_prefix(input);
        assert_eq!(result, "flower".to_string());
    }
    #[test]
    fn case5() {
        let input = vec!["ab".to_string(), "a".to_string()];
        let result = longest_common_prefix(input);
        assert_eq!(result, "a".to_string());
    }
}
