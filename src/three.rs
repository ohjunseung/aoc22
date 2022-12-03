pub fn run(input: &str) -> u32 {
    let mut items: Vec<char> = vec![];

    for line in input.lines() {
        let char_list: Vec<char> = line.chars().collect();
        let sacks: Vec<&[char]> = char_list.chunks(char_list.len() / 2).collect();
        if sacks.len() != 2 {
            continue;
        }
        for c in sacks[0] {
            if sacks[1].contains(c) && !items.contains(c) {
                items.push(*c)
            }
        }
    }
    items.iter().map(|c| char_to_prio(*c)).sum()
}

pub fn char_to_prio(c: char) -> u32 {
    if c.is_ascii_alphabetic() {
        if c.is_lowercase() {
            c as u32 - 96
        } else {
            c as u32 - 64 + 26
        }
    } else {
        0
    }
}
