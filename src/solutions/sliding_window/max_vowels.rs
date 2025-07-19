struct Solution;

// 1456. Maximum Number of Vowels in a Substring of Given Length
// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length
impl Solution {
    #[inline]
    pub fn is_vowel(str: &char) -> bool {
        matches!(str, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        if s.len() == 0 || k == 0 {
            return 0;
        }

        let k = k as usize;

        if k > s.len() {
            return 0;
        }

        // s is the given string, k is the length
        let s_vec = s.chars().collect::<Vec<char>>();

        // max chars
        let mut max = 0;

        // sliding window
        // similar to find_max_average, but the max is computed using num of vowels.
        for i in 0..k {
            let is_vowel = Self::is_vowel(&s_vec[i]) as i32;
            max += is_vowel;
        }

        // current
        let mut current = max;

        // for loop to iterate all
        // T: O(n), S: O(1)
        for i in k..s.len() {
            let left = Self::is_vowel(&s_vec[i - k]) as i32;
            let next = Self::is_vowel(&s_vec[i]) as i32;

            current = current - left + next;
            max = max.max(current);
        }

        max
    }
}
