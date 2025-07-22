use std::collections::HashMap;

struct Solution;

// 1657. Determine if Two Strings Are Close
// https://leetcode.com/problems/determine-if-two-strings-are-close
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut freq_1 = vec![0; 26];
        let mut freq_2 = vec![0; 26];
        for c in word1.chars() {
            // get its ascii code and update its frequency
            freq_1[c as usize - 'a' as usize] += 1;
        }
        for c in word2.chars() {
            freq_2[c as usize - 'a' as usize] += 1;
        }

        for i in 0..26 {
            // compare if some char exists in another, if not, then string can not be attained
            if freq_1[i] == 0 && freq_2[i] != 0 || freq_1[i] != 0 && freq_2[i] == 0 {
                return false;
            }
        }

        // we sort frequencies distribution
        freq_1.sort_unstable();
        freq_2.sort_unstable();

        // compare our distribution
        // if they equals, then we can atain
        freq_1 == freq_2
    }
}
