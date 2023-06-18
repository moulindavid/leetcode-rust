struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut upper_bound = *piles.iter().max().unwrap() as usize;
        let mut lower_bound = 1;
        //minimum number of banana to eat in one hour.
        let mut result = upper_bound;
        while lower_bound <= upper_bound {
            let k = lower_bound + (upper_bound - lower_bound) / 2;

            let mut hours_to_eat: usize = 0;

            for bananas in piles.iter() {
                hours_to_eat += ((bananas - 1) as usize / k) + 1;
            }

            if hours_to_eat <= h as usize {
                result = result.min(k);
                upper_bound = k - 1;
            } else {
                lower_bound = k + 1;
            }
        }
        return result as i32;
    }
}

fn main() {
    dbg!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
    dbg!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6));
    dbg!(Solution::min_eating_speed(
        vec![805306368, 805306368, 805306368],
        1000000000
    ));
}
