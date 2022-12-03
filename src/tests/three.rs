#[test]
fn priority() {
    assert_eq!(0, crate::three::char_to_prio('0'));
    assert_eq!(0, crate::three::char_to_prio('?'));

    for (i, c) in ('a'..='z').enumerate() {
        assert_eq!((i + 1) as u32, crate::three::char_to_prio(c));
    }
    for (i, c) in ('A'..='Z').enumerate() {
        assert_eq!((i + 27) as u32, crate::three::char_to_prio(c));
    }
}
#[test]
fn run() {
    let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(crate::three::run(input), 157);
}
