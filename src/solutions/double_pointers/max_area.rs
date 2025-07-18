struct Solution;

// 11. Container With Most Water
// https://leetcode.com/problems/container-with-most-water
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();

        if n == 0 {
            return 0;
        }

        if n == 1 {
            return height[0];
        }

        let mut i = 0;
        let mut j = n - 1;
        let mut area = 0;

        // until i and j meet
        while i < j {
            let current = height[i];
            let current_j = height[j];

            let diff = j - i;
            let current_area = diff as i32 * current.min(current_j);

            // only update if we found a larger area
            if current_area > area {
                area = current_area;
            }

            // println!("area : {current_area}");

            // println!("i : {i}");

            // println!("j : {j}");

            // think:
            // advance pointer if some is less than the other
            // the logic is to find indexes with the maximum height
            if current < current_j {
                i += 1;
            } else {
                j -= 1;
            }
        }

        area
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
