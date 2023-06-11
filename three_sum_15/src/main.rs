struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort();

        for (i, &a) in nums.iter().enumerate() {
            if i > 0 && a == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = a + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![a, nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                    continue;
                }
                if sum < 0 {
                    left += 1;
                    continue;
                }
                if sum > 0 {
                    right -= 1;
                }
            }
        }

        return result;
    }
}

fn main() {
    dbg!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    dbg!(Solution::three_sum(vec![-1, 0, 0, 0]));
}
