struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        return strs
            .iter()
            .map(|word| format!("{}#{}", word.len(), word))
            .collect::<Vec<String>>()
            .join("");
    }
    pub fn decode(s: &str) -> Vec<String> {
        let mut res = Vec::new();
        //Keep tracks of current position in the string.
        let mut i = 0;

        while i < s.len() {
            let mut j = i;
            while s.chars().nth(j) != Some('#') {
                j += 1;
            }
            //Size of the jump.
            let length = s[i..j].parse::<usize>().unwrap();
            res.push(s[j + 1..j + 1 + length].to_string());
            //Skip each words
            i = j + 1 + length;
        }

        res
    }
}

fn main() {
    dbg!(Solution::encode(vec![
        "eat".to_owned(),
        "tea".to_owned(),
        "tan".to_owned(),
        "ate".to_owned(),
        "nat".to_owned(),
        "asdasdas".to_owned()
    ]));
    dbg!(Solution::decode("3#eat3#tea3#tan3#ate3#nat8#asdasdas"));
}
