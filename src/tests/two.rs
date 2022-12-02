#[test]
fn check_score() {
    use crate::two::{get_score, Hand};

    let strategy = vec![
        (Hand::Rock, Hand::Paper),
        (Hand::Paper, Hand::Rock),
        (Hand::Scissor, Hand::Scissor),
    ];

    assert_eq!(get_score(strategy), 15);
}

#[test]
fn check_parsing() {
    use crate::two::{parse, Hand};
    let a = vec!["A Y", "B X", "C Z"];
    let b = vec!["a a"];
    let c = vec!["asem"];
    let d = vec![];

    assert_eq!(
        parse(a),
        vec![
            (Hand::Rock, Hand::Paper),
            (Hand::Paper, Hand::Rock),
            (Hand::Scissor, Hand::Scissor)
        ]
    );
    assert_eq!(parse(b), vec![(Hand::Nil, Hand::Nil)]);
    assert_eq!(parse(c), vec![(Hand::Nil, Hand::Nil)]);
    assert_eq!(parse(d), vec![]);
}
#[test]
fn run() {
    use crate::two::{get_score, parse};
    let strategy1 = vec!["A Y", "B X", "C Z"];
    let strategy2 = vec!["A Z", "A X", "C Z", "C X"];
    let strategy3 = vec!["C X", "B X", "A Z", "A Y"];

    let strategy1 = parse(strategy1);
    let strategy2 = parse(strategy2);
    let strategy3 = parse(strategy3);
    assert_eq!(get_score(strategy1), 15);
    assert_eq!(get_score(strategy2), 20);
    assert_eq!(get_score(strategy3), 19);
}
