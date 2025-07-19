struct Solution;

// 1004. Max Consecutive Ones III
// https://leetcode.com/problems/max-consecutive-ones-iii
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if nums.is_empty() {
            return 0;
        }

        let mut k = k;
        let mut i = 0;
        let mut j = 0;

        // think:
        // we iterate through the array of items
        // so we have double pointers i and j
        // k as a capacity of zeros we can flip
        // so, we start from left to right
        // if we encounter a zero, k should be decremented
        // there would be a case when we would have exhausted all the capacity,
        // in this case, we slide the window to the right (unflip the most left zero)
        // finally we just return the length of the window, which is the length of the most consecutive ones
        while j < n {
            // capacity-1 if need to flip
            if nums[j] == 0 {
                k -= 1;
            }

            if k < 0 {
                // refill
                if nums[i] == 0 {
                    k += 1;
                }
                i += 1;
            }
            j += 1;
        }

        // while loop, so j - i
        (j - i) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_ones() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
        assert_eq!(
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
