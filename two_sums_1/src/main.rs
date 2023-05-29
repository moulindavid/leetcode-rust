use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum_n(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let index = i as i32;
            let complement = target - num;
            if map.contains_key(&complement) {
                return vec![index, map[&complement]];
            }
            map.insert(*num, index);
        }
        return vec![0, 1];
    }

    pub fn two_sum_n2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![0, 1];
    }
}

fn main() {
    dbg!(Solution::two_sum_n(vec![2, 8, 7, 1], 9));
    dbg!(Solution::two_sum_n2(vec![2, 8, 7, 1], 9));
}
