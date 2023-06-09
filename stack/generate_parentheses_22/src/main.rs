struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
            let mut res = vec![];

            if open == 0 && close == 0 {
                return vec![s];
            }

            if open > 0 {
                res.append(&mut back_track(s.clone() + "(", open - 1, close + 1));
            }

            if close > 0 {
                res.append(&mut back_track(s.clone() + ")", open, close - 1));
            }
            return res;
        }

        return back_track("".to_string(), n, 0);
    }
}

fn main() {
    dbg!(Solution::generate_parenthesis(1));
    dbg!(Solution::generate_parenthesis(3));
}
