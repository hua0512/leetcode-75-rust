struct Solution;

// 392. Is Subsequence
// https://leetcode.com/problems/is-subsequence
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }

        if s.len() == 0 {
            return true;
        }

        // s index
        let mut i = 0;

        let chars_s = s.chars().collect::<Vec<char>>();
        for t in t.chars() {
            let find = chars_s[i];
            if t == find {
                i += 1;
            }
            if i >= s.len() {
                break;
            }
        }

        i == s.len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("b".to_string(), "abc".to_string()),
            true
        );
    }
}
