struct Solution;

// 735. Asteroid Collision
// https://leetcode.com/problems/asteroid-collision
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = vec![];

        // think:
        // asteroids collision happens when two asteroids are moving towards each other
        // which means they have opposite signs, so the first asteroid should be > 0 (moving right) and the second should be < 0 (moving left)
        // after collision, the smaller one will be destroyed, so we need to compare their absolute values and remove the smaller one
        // if both asteroids have the same size, they will both be destroyed
        for current in asteroids.iter() {
            let mut skip = false;

            while !stack.is_empty() && !skip {
                let last = *stack.last().unwrap();

                // potential collision scenario
                if last > 0 && *current < 0 {
                    // destroy both
                    if last.abs() == current.abs() {
                        stack.pop();
                        skip = true;
                    } else if last.abs() > current.abs() {
                        // Current asteroid destroyed, last survives
                        skip = true;
                    } else {
                        // Current asteroid destroys last, continue checking
                        stack.pop();
                    }
                } else {
                    // no collision
                    break;
                }
            }

            if !skip {
                stack.push(*current);
            }
        }

        stack
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_asteroid_collision() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }
}
