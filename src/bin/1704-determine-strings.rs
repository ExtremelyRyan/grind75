// https://leetcode.com/problems/determine-if-string-halves-are-alike/

fn main() {  
    let samplestr = String::from("textbook");
    println!("The word is {samplestr}.");
    let (a, b) = parse(&samplestr);
    let a_result = find_vowels(&a);
    let b_result = find_vowels(&b);
    if a_result == b_result {
        println!("{a} and {b} are alike.");
        println!("vowels in {a}: {a_result}, vowels in {b}: {b_result}.\n")
    }
    else{ 
        println!("{a} and {b} are NOT alike.");
        println!("vowels in {a}: {a_result}, vowels in {b}: {b_result}.\n")
    }
}

fn parse(input: &String) -> (String, String) {
    let (a,b) = input.split_at(input.len() / 2);
    return (a.to_owned(), b.to_owned());
    
}

fn find_vowels(a: &String) -> i32 { 
    let mut v_count = 0;

    for c in a.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => v_count += 1,
            'A' | 'E' | 'I' | 'O' | 'U' => v_count += 1,
            _ => continue,
        }
    }
    v_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let (a,b) = parse(&"book".to_string());
        assert_eq!(a, "bo");
        assert_eq!(b, "ok");
    }
    #[test]
    fn test_find_vowels() {
        let (a,b) = parse(&"book".to_string());
        let a_result = find_vowels(&a);
        let b_result = find_vowels(&b);
        assert_eq!(a_result, 1);
        assert_eq!(b_result, 1);
    }
}