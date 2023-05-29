use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut unique = HashSet::with_capacity(nums.len());
        // HashSet.insert will return true if the inserted value does not exist in the hash set.
        return !&nums.into_iter().all(|v| unique.insert(v));
    }
}

fn main() {
    dbg!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    dbg!(Solution::contains_duplicate(vec![
        1, 1, 1, 3, 3, 4, 3, 2, 4, 2
    ]));
}
