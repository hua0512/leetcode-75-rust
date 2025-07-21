use std::collections::HashMap;

struct Solution;

// 2215. Find the Difference of Two Arrays
// https://leetcode.com/problems/find-the-difference-of-two-arrays
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut not_in_nums_1 = vec![];
        let mut not_in_nums_2 = vec![];

        let mut hashmap_1: HashMap<i32, i32> = HashMap::new();
        let mut hashmap_2: HashMap<i32, i32> = HashMap::new();

        for num in nums1.iter() {
            hashmap_1.insert(*num, 1);
        }

        for num in nums2.iter() {
            hashmap_2.insert(*num, 2);
        }

        for num in nums1 {
            if !hashmap_2.contains_key(&num) {
                not_in_nums_1.push(num);
                // mark as processed
                hashmap_2.insert(num, 2);
            }
        }

        for num in nums2 {
            if !hashmap_1.contains_key(&num) {
                not_in_nums_2.push(num);
                // mark as processed
                hashmap_1.insert(num, 2);
            }
        }

        vec![not_in_nums_1, not_in_nums_2]
    }
}
