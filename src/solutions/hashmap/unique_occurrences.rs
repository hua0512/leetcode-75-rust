use std::collections::{HashMap, HashSet};

struct Solution;

// 1207. Unique Number of Occurrences
// https://leetcode.com/problems/unique-number-of-occurrences
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occ = HashMap::new();

        for num in arr.iter() {
            *occ.entry(*num).or_insert(0) += 1;
        }

        let mut unique_set = HashSet::new();

        for num in occ.values() {
            if unique_set.contains(num) {
                return false;
            }
            unique_set.insert(*num);
        }

        true
    }
}
