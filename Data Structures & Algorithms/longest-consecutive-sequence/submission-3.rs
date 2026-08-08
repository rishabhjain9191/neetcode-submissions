use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut past_sequences: HashMap<i32, i32> = HashMap::new();
        let mut max_len = 0;

        for &num in &num_set {
            let mut seq_length = 1;
            let mut next_num = num + 1;

            // Direct memoization check using if-let to avoid double lookup
            if let Some(&cached_len) = past_sequences.get(&next_num) {
                seq_length += cached_len;
            } else {
                // Count consecutive numbers sequentially
                while num_set.contains(&next_num) {
                    if let Some(&cached_len) = past_sequences.get(&next_num) {
                        seq_length += cached_len;
                        break;
                    }
                    seq_length += 1;
                    next_num += 1;
                }
            }

            // Cache the sequence length for the starting number
            past_sequences.insert(num, seq_length);
            max_len = max_len.max(seq_length);
        }

        max_len
    }
}