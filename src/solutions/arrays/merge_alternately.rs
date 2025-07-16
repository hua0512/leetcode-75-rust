pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let i1 = word1.len();
        let i2 = word2.len();

        let mut new_string = String::with_capacity(i1 + i2);
        let mut i = 0;
        while i < i1 || i < i2 {
            if i < i1 {
                new_string.push(word1.chars().nth(i).unwrap());
            }

            if i < i2 {
                new_string.push(word2.chars().nth(i).unwrap());
            }

            i += 1;
        }
        new_string
    }
}
