#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hand {
    Nil,
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum Score {
    Lost,
    Draw = 3,
    Win = 6,
}

/// Only calculate total score for us(second term in tuple)
pub fn get_score(strategy: Vec<(Hand, Hand)>) -> u32 {
    let mut score = 0;
    for round in strategy {
        score += round.1 as u32;
        score += check_win_score(round) as u32;
    }
    score
}

/// Only calculate win score for us(second term in tuple)
fn check_win_score(round: (Hand, Hand)) -> Score {
    match round {
        (Hand::Rock, Hand::Paper) => Score::Win,
        (Hand::Paper, Hand::Scissor) => Score::Win,
        (Hand::Scissor, Hand::Rock) => Score::Win,
        (Hand::Paper, Hand::Rock) => Score::Lost,
        (Hand::Scissor, Hand::Paper) => Score::Lost,
        (Hand::Rock, Hand::Scissor) => Score::Lost,
        (Hand::Nil, Hand::Nil) => Score::Lost,
        _ => Score::Draw,
    }
}

pub fn parse(strategy: Vec<&str>) -> Vec<(Hand, Hand)> {
    let result = strategy
        .iter()
        .map(|x| {
            let tmp: Vec<&str> = x.split(" ").collect();
            if tmp.len() < 2 {
                ("", "")
            } else {
                (tmp[0], tmp[1])
            }
        })
        .map(|x| match x {
            (a, x) => (
                match a {
                    "A" => Hand::Rock,
                    "B" => Hand::Paper,
                    "C" => Hand::Scissor,
                    _ => Hand::Nil,
                },
                match x {
                    "X" => Hand::Rock,
                    "Y" => Hand::Paper,
                    "Z" => Hand::Scissor,
                    _ => Hand::Nil,
                },
            ),
        })
        .collect::<Vec<(Hand, Hand)>>();

    result
}
