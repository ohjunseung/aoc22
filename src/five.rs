type Stack = Vec<char>;

struct Procedure {
    take: u32,
    from: u32,
    to: u32,
}

impl Procedure {
    fn new() -> Self {
        Self {
            take: 0,
            from: 0,
            to: 0,
        }
    }
}

pub fn run(input: &str) -> String {
    let (mut stacks, proc) = parse(input);
    for proc in proc {
        for _ in 0..proc.take {
            if let Some(c) = stacks[(proc.from - 1) as usize].pop() {
                stacks[(proc.to - 1) as usize].push(c);
            }
        }
    }

    let mut result = String::new();
    for stack in stacks {
        if let Some(c) = stack.last() {
            result.push(*c);
        }
    }

    result
}
fn parse(input: &str) -> (Vec<Stack>, Vec<Procedure>) {
    let splitted: Vec<&str> = input.split("\n\n").collect();
    if splitted.len() != 2 {
        return (vec![], vec![]);
    }

    let stack = splitted[0];
    let proc = splitted[1];

    let mut stack_1: Stack = vec![];
    let mut stack_2: Stack = vec![];
    let mut stack_3: Stack = vec![];

    for line in stack.lines() {
        if let Some(c) = line.chars().nth(1) {
            stack_1.push(c);
        };
        if let Some(c) = line.chars().nth(5) {
            stack_2.push(c);
        };
        if let Some(c) = line.chars().nth(9) {
            stack_3.push(c);
        };
    }

    stack_1 = stack_1
        .into_iter()
        .filter(|x| x.is_ascii_alphabetic())
        .rev()
        .collect();
    stack_2 = stack_2
        .into_iter()
        .filter(|x| x.is_ascii_alphabetic())
        .rev()
        .collect();
    stack_3 = stack_3
        .into_iter()
        .filter(|x| x.is_ascii_alphabetic())
        .rev()
        .collect();

    let mut parsed_proc: Vec<Procedure> = vec![];

    for line in proc.lines() {
        let mut proc = Procedure::new();
        let mut word_iter = line.split(' ');

        word_iter.next(); //move
        if let Some(take) = word_iter.next() {
            if let Ok(n) = take.parse::<u32>() {
                proc.take = n
            };
        };
        word_iter.next(); //from
        if let Some(from) = word_iter.next() {
            if let Ok(n) = from.parse::<u32>() {
                proc.from = n
            };
        };
        word_iter.next(); //to
        if let Some(to) = word_iter.next() {
            if let Ok(n) = to.parse::<u32>() {
                proc.to = n
            };
        };

        parsed_proc.push(proc);
    }

    (vec![stack_1, stack_2, stack_3], parsed_proc)
}
