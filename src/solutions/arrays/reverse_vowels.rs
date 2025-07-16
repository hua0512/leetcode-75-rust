struct Solution;

// 345. Reverse Vowels of a String
// https://leetcode.com/problems/reverse-vowels-of-a-string/
impl Solution {
    pub fn is_vowel(s: &char) -> bool {
        matches!(s.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    }

    pub fn reverse_vowels(s: String) -> String {
        let len = s.len();
        let mut result = s.chars().collect::<Vec<char>>();

        let mut i = 0;
        let mut j = len - 1;

        while i < j {
            let char = result[i];
            if Solution::is_vowel(&char) {
                let char_j = result[j];
                if Solution::is_vowel(&char_j) {
                    result[i] = char_j;
                    result[j] = char;
                    i += 1;
                }
                j -= 1;
            } else {
                i += 1;
            }
        }

        result.into_iter().collect()
    }
}
