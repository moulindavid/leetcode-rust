struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //Edge cases, negatives, if last digit is 0.
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut y = x;
        let mut reverted: i32 = 0;
        while y > reverted {
            reverted = reverted * 10 + y % 10;
            y /= 10;
        }
        return y == reverted || y == reverted / 10;
    }
    pub fn is_palindrome_faster(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x = x.to_string();
        let n = x.len();
        for i in 0..&n / 2 {
            //Comparing each number side by side.
            if x.chars().nth(i).unwrap() != x.chars().nth(&n - 1 - i).unwrap() {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    dbg!(Solution::is_palindrome(12321));
}
