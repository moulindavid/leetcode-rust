use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut count_map = HashMap::new();

        s.chars()
            .for_each(|c| *count_map.entry(c).or_insert(0) += 1);
        t.chars()
            .for_each(|c| *count_map.entry(c).or_insert(0) -= 1);

        return count_map.into_values().all(|v| v == 0);
    }
}

fn main() {
    dbg!(Solution::is_anagram(
        "anagram".to_owned(),
        "nagaram".to_owned()
    ));
    dbg!(Solution::is_anagram("rat".to_owned(), "car".to_owned()));
    dbg!(Solution::is_anagram(
        "ratsdf".to_owned(),
        "ratsdfz".to_owned()
    ));
}
