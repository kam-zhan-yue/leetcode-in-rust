use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        // Create a hashmap with the alphabet array as a key
        for s in strs {
            let mut alphabet = vec![0; 26];
            for c in s.chars() {
                let pos = c as usize - 'a' as usize;
                alphabet[pos as usize] += 1;
            }

            map.entry(alphabet)
                .and_modify(|array| array.push(s.clone()))
                .or_insert(vec![s]);
        }

        return map.into_values().collect();
    }
}

fn main() {
    let v = vec!["ant".to_string(), "tan".to_string(), "bat".to_string()];
    println!("{:?}", Solution::group_anagrams(v));
}
