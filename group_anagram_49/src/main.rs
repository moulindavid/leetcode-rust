use std::{collections::HashMap, vec};

struct Solution;

//Number of letters
const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<[u8; N_LETTERS], Vec<String>> = HashMap::new();

        for ele in strs {
            let mut count = [0; N_LETTERS];
            for char in ele.chars() {
                let char_index = char as usize - 'a' as usize;
                count[char_index] += 1;
            }

            result.entry(count).or_insert(Vec::new()).push(ele);
        }

        return result.values().cloned().collect();
    }
}

fn main() {
    dbg!(Solution::group_anagrams(vec![
        "eat".to_owned(),
        "tea".to_owned(),
        "tan".to_owned(),
        "ate".to_owned(),
        "nat".to_owned(),
        "bat".to_owned()
    ]));
}
