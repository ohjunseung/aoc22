pub fn run(input: &str) -> u32 {
    // index outofbound access lingering here, oops!
    let mut count = 0;
    for line in input.lines() {
        let elves: Vec<(u32, u32)> = line
            .split(',')
            .map(|x| {
                let section: Vec<u32> = x.split('-').map(|x| x.parse().unwrap_or(0u32)).collect();
                (section[0], section[1])
            })
            .collect();

        if elves[0].0 <= elves[1].0 && elves[0].1 >= elves[1].1 {
            count += 1;
        } else if elves[1].0 <= elves[0].0 && elves[1].1 >= elves[0].1 {
            count += 1
        }
    }
    count
}
