struct Solution;
// 2390. Removing Stars From a String
// https://leetcode.com/problems/removing-stars-from-a-string/
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result = String::new();

        // T: O(n), S: O(n) (0(1) if result string is not considered)
        for c in s.chars() {
            if c == '*' {
                result.pop();
            } else {
                result.push(c);
            }
        }
        result
    }
}
