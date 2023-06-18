struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        //Create vector of tuples i32,i32.

        let mut fleets = 0;
        let mut data = position
            .into_iter()
            .zip(speed.into_iter())
            .collect::<Vec<_>>();
        data.sort_unstable_by_key(|(x, _)| -x);
        //No fleets arrived in 0.
        let mut previous_time = 0f32;
        //We take the problem from the car starting in the last position.
        for (pos, velocity) in data.into_iter() {
            let destination_time = ((target - pos) as f32) / (velocity as f32);
            if destination_time > previous_time {
                fleets += 1;
                previous_time = destination_time;
            }
        }
        return fleets;
    }
}

fn main() {
    dbg!(Solution::car_fleet(
        12,
        vec![10, 8, 0, 5, 3],
        vec![2, 4, 1, 1, 3]
    ));
}
