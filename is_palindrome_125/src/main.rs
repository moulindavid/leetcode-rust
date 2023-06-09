struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let len = chars.len();

        //Need to account for odd number, and upper bound is exclusive.
        (0..len / 2 + len % 2)
            .into_iter()
            .map(|i| chars[i])
            .eq((len / 2..len).rev().map(|i| chars[i]))
    }
}

fn main() {
    dbg!(Solution::is_palindrome("a".to_owned()));
    dbg!(Solution::is_palindrome("kayacak".to_owned()));
    dbg!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".to_owned()
    ));
}
