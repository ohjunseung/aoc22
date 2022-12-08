pub fn run(input: &str) -> u32 {
    let trees = parse(input);
    let mut visible = (2 * trees.len() + 2 * trees[0].len()) - 4;

    for (j, line) in trees.iter().enumerate() {
        if j == 0 || j == trees.len() - 1 {
            continue;
        };
        for (i, v) in line.iter().enumerate() {
            if i == 0 || i == line.len() - 1 {
                continue;
            }
            for tmp in (i..line.len()).rev() {
                if line[tmp] >= *v {
                    break;
                }
                if tmp == i + 1 {
                    visible += 1
                }
            }
            for tmp in 0..i {
                if line[tmp] >= *v {
                    break;
                }
                if tmp == i - 1 {
                    visible += 1
                }
            }

            for tmp in (j..trees.len()).rev() {
                if trees[j][i] >= *v {
                    break;
                }
                if tmp == j + 1 {
                    visible += 1
                }
            }
            for tmp in 0..j {
                if trees[j][i] >= *v {
                    break;
                }
                if tmp == j - 1 {
                    visible += 1
                }
            }
        }
    }

    visible as u32
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| (c as u32) - 48)
                .collect()
        })
        .collect()
}
