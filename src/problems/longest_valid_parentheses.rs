use crate::common::Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut valids: Vec<(i32, i32)> = vec![];
        let mut op_stack: Vec<i32> = vec![];

        for (i, ch) in s.bytes().enumerate() {
            let index: i32 = i as i32;

            if ch == b'(' {
                op_stack.push(index);
            } else if ch == b')' {
                match op_stack.pop() {
                    Some(x) => {
                        valids.push(match valids.iter().find(|&&valid| valid.1 == x) {
                            Some(valid) => (valid.0, index + 1),
                            None => (x, index + 1),
                        });
                    }
                    None => {}
                };
            }
        }

        valids.iter().map(|x| x.1 - x.0).max().unwrap_or(0)
    }
}

pub fn test() {
    let test_func = Solution::longest_valid_parentheses;

    assert_eq!(test_func(String::from("(((")), 0);
    assert_eq!(test_func(String::from("(()")), 2);

    assert_eq!(test_func(String::from(")()())")), 4);
    assert_eq!(test_func(String::from(")(()())")), 6);
    assert_eq!(test_func(String::from("())))((()()))")), 8);
    assert_eq!(test_func(String::from("()(()()(()")), 4);

    assert_eq!(test_func(String::from("((()((()")), 2);
    assert_eq!(test_func(String::from("()(()")), 2);
    assert_eq!(test_func(String::from("()(()(()")), 2);

    assert_eq!(test_func(String::from("()(()()")), 4);
    assert_eq!(test_func(String::from("()(()()(())")), 8);
    assert_eq!(test_func(String::from("()(()()(()(())")), 6);
    assert_eq!(test_func(String::from("()(()()(()(())))")), 16);

    assert_eq!(test_func(String::from(")()(()")), 2);
    assert_eq!(test_func(String::from(")()(()()")), 4);
    assert_eq!(test_func(String::from(")()(()()(())")), 8);
    assert_eq!(test_func(String::from(")()(()()(()(())")), 6);
    assert_eq!(test_func(String::from(")()(()()(()(())))")), 16);
}
