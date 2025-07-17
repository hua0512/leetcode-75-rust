struct Solution;

// 443. String compression
// https://leetcode.com/problems/string-compression
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.is_empty() {
            return 0;
        }

        if chars.len() == 1 {
            return 1;
        }

        // read pointer
        let mut i = 0;
        // write pointer
        let mut j = 0;

        while i < chars.len() {
            let current_char = chars[i];
            let mut count = 1;

            // count for same chars
            while i + count < chars.len() && chars[i + count] == current_char {
                count += 1;
            }

            // write the character
            chars[j] = current_char;
            j += 1;

            // write if count is greater than 1
            if count > 1 {
                let count_str = count.to_string();
                for digit in count_str.chars() {
                    chars[j] = digit;
                    j += 1;
                }
            }

            // skip the count
            i += count;
        }

        j as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
            6
        );
        assert_eq!(Solution::compress(&mut vec!['a']), 1);
        assert_eq!(Solution::compress(&mut vec![]), 0);
        assert_eq!(
            Solution::compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ]),
            4
        );
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'a', 'b', 'b', 'a', 'a']),
            6
        );

        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'a', 'a', 'a', 'b']),
            3
        );
    }
}
