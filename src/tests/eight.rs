#[test]
fn run() {
    let input = r"30373
25512
65332
33549
35390";

    assert_eq!(crate::eight::run(input), 21);
}
