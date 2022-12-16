

// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. 
// The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

// Increment the large integer by one and return the resulting array of digits.

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let result: Vec<i32> = Vec::new();    

    let mut last = digits.get(digits.len() -1).unwrap().to_owned();

    match last {
        0..=8 => last += 1,
        9 => {
            last = 0;
            *digits.get_mut(digits.len() - 2).unwrap() += 1;
        }
        _ => panic!(),
    }


    let mut digits = digits;
    for n in digits.iter_mut().rev() {
        let sum = *n + 1;
        *n = sum % 10;
        if sum < 10 {
            return digits;
        }
    }

    [&vec![1], &digits[..]].concat()
}


#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn case1() { 
        let v = vec![1,2,3];
        assert_eq!( plus_one(v),  vec![1,2,4]);
    }

    #[test]
    fn case2() { 
        let v = vec![4,3,2,1];
        assert_eq!( plus_one(v),  vec![4,3,2,2]);
    }

    #[test]
    fn case3() { 
        let v = vec![9];
        assert_eq!( plus_one(v),  vec![1,0]);
    }
}