struct Solution;

/**
 * 605. Can Place Flowers
 * You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.

Example 1:

Input: flowerbed = [1,0,0,0,1], n = 1
Output: true

Example 2:

Input: flowerbed = [1,0,0,0,1], n = 2
Output: false


Constraints:

    1 <= flowerbed.length <= 2 * 104
    flowerbed[i] is 0 or 1.
    There are no two adjacent flowers in flowerbed.
    0 <= n <= flowerbed.length

 */
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut can_place_n = 0;

        let mut previous_space = 0;
        for &space in &flowerbed {
            if previous_space == 0 && space == 0 {
                can_place_n += 1;
                previous_space = 2;
            } else if previous_space == 2 && space == 1 {
                // 2 - we think about placing flower in the previous plot, but then we see there is a flower already
                // in the adjacent flow - we backtrack.
                can_place_n -= 1;
                previous_space = space;
            } else {
                previous_space = space;
            }
        }

        can_place_n >= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }
}
