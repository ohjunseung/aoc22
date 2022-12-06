#[test]
fn run() {
    use crate::six::run;
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(run(input), 5);
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(run(input), 6);
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(run(input), 10);
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(run(input), 11);
}
