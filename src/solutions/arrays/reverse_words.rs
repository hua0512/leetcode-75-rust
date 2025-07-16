struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let mut result = String::new();
        let mut i = len;

        while i > 0 {
            // remove trailing spaces
            while i > 0 && chars[i - 1] == ' ' {
                i -= 1;
            }

            // If we've processed all characters, break
            if i == 0 {
                break;
            }

            // end of the word
            let end = i - 1;

            while i > 0 && chars[i - 1] != ' ' {
                i -= 1;
            }

            // start of the word
            let start = i;

            // add space before word if result is not empty
            if !result.is_empty() {
                result.push(' ');
            }

            // add slice
            for j in start..=end {
                result.push(chars[j]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello"
        );

        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a"
        );
        assert_eq!(Solution::reverse_words("a".to_string()), "a");
        assert_eq!(Solution::reverse_words("I     ueG7yY0Tgo i8Q9SUKyLz    vk4zC 9p      eDpBX96MRL  IhzW  K   T    6FzR0 yKYgG".to_string()), "yKYgG 6FzR0 T K IhzW eDpBX96MRL 9p vk4zC i8Q9SUKyLz ueG7yY0Tgo I");
    }
}
