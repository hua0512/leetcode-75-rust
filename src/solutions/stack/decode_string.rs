use std::collections::VecDeque;

struct Solution;

// 394. Decode String
// https://leetcode.com/problems/decode-string
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result = String::new();
        let mut deque = VecDeque::new();

        // we may use double stack to solve this problem
        // T: O(n + m), S: O(n + m)
        for char in s.chars() {
            // found delimiter
            if char == ']' {
                let mut multiplier = 0;
                while let Some(last_char) = result.pop() {
                    if last_char == '[' {
                        break;
                    }
                    deque.push_front(last_char);
                }
                // println!("string : {s}");

                // collect digits in reverse order
                let mut power = 1;
                while let Some(last_char) = result.pop() {
                    if last_char.is_digit(10) {
                        multiplier += last_char.to_digit(10).unwrap() * power;
                        power *= 10;
                    } else {
                        result.push(last_char);
                        break;
                    }
                }

                // println!("multiplier : {multiplier}");
                for _ in 0..multiplier {
                    for i in 0..deque.len() {
                        result.push(deque[i]);
                    }
                }
                deque.clear();
            } else {
                result.push(char);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc");
        assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef"
        );
        assert_eq!(
            Solution::decode_string("100[leetcode]".to_string()),
            "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode"
        );
    }
}
