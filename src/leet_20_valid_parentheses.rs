
/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', 
determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.
*/

pub fn is_valid(s: String) -> bool {
    // our stack for matching end parens
    let mut stack = "".to_string();

    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '}' | ']' | ')' => {
                if let Some(matching) = stack.pop() {
                    if matching != ch {
                        return false;
                    }
                } else {
                    // stack is empty
                    return false;
                }
            }
            _ => return false,
        }
        
    }
    // our stack should be empty here.
    stack.len() == 0

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string() {
        assert!( is_valid("".to_string()));
    }
    #[test]
    fn example1() {
        assert!( is_valid("()".to_string()));
    }
    #[test]
    fn example2() {
        assert!( is_valid("()[]{}".to_string()));
    }
    #[test]
    fn example3() {
        assert!( !is_valid("(]".to_string()));
    }

}
