use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map_frequence = HashMap::new();

        for num in nums {
            *map_frequence.entry(num).or_insert(0) += 1;
        }

        let mut count_vec: Vec<(&i32, &i32)> = map_frequence.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(&a.1));

        let mut result: Vec<i32> = vec![];

        for i in 0..k {
            result.push(*count_vec[i as usize].0);
        }
        return result;
    }
}

fn main() {
    dbg!(Solution::top_k_frequent(vec![1, 2, 3, 1], 1));
    dbg!(Solution::top_k_frequent(
        vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        2
    ));
}
