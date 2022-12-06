use std::collections::HashSet;

pub fn run(input: &str) -> usize {
    let mut result = 0;
    for window in input.chars().collect::<Vec<char>>().windows(4) {
        let mut tmp = window.to_vec();
        let mut set = HashSet::new();
        tmp.retain(|c| set.insert(*c));
        if tmp.len() == 4 {
            result += 4;
            break;
        }
        result += 1;
    }
    result
}
