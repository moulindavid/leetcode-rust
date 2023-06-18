struct Solution;

impl Solution {
    pub fn is_palindrome_iter(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let len = chars.len();

        (0..len / 2)
            .into_iter()
            .map(|i| chars[i])
            .eq((len / 2..len).rev().map(|i| chars[i]))
    }
    pub fn is_palindrome_without_iter(s: String) -> bool {
        let mut chars: Vec<char> = Vec::new();
        for c in s.chars() {
            if c.is_ascii_alphanumeric() {
                chars.push(c.to_ascii_lowercase());
            }
        }
        let len = chars.len();

        for i in 0..len / 2 + len % 2 {
            if chars[i] != chars[len - i - 1] {
                return false;
            }
        }

        return true;
    }
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();

        let len = s.len();

        for i in 0..(len / 2) {
            if s[i] != s[len - i - 1] {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    dbg!(Solution::is_palindrome("a".to_owned()));
    dbg!(Solution::is_palindrome("kayacak".to_owned()));
    dbg!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".to_owned()
    ));
}
