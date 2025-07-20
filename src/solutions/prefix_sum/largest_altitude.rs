struct Solution;

// 1732. Find the Highest Altitude
// https://leetcode.com/problems/find-the-highest-altitude/
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;

        let mut acc = 0;

        for g in gain {
            acc += g;
            if acc > max {
                max = acc;
            }
        }

        max
    }
}
