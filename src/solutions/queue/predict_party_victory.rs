use std::collections::VecDeque;

struct Solution;

// 649. Dota2 Senate
// https://leetcode.com/problems/dota2-senate
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant_queue = VecDeque::with_capacity(senate.len() / 2);
        let mut dire_queue = VecDeque::with_capacity(senate.len() / 2);

        // think:
        // we use two queues to track indices of senators
        // and we determine the winner by comparing the indices of them
        // each round, the senator with smaller index eliminates the other, and goes to the back of the queue for next round
        // finally, if the queue is empty, the party loses
        for (i, c) in senate.chars().enumerate() {
            if c == 'R' {
                radiant_queue.push_back(i);
            } else {
                dire_queue.push_back(i);
            }
        }

        while !radiant_queue.is_empty() && !dire_queue.is_empty() {
            let r = radiant_queue.pop_front().unwrap();
            let d = dire_queue.pop_front().unwrap();

            // smaller index one eliminates the loser, and goes to back of queue for next round
            if r < d {
                radiant_queue.push_back(r + senate.len());
            } else {
                dire_queue.push_back(d + senate.len());
            }
        }

        if radiant_queue.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}
