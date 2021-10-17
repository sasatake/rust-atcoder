use cli_test_dir::*;

#[test]
fn abc087_b_01() {
    let testdir = TestDir::new("abc087_b", "abc087_b_01");
    let input = r"2
    2
    2
    100";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc087_b_02() {
    let testdir = TestDir::new("abc087_b", "abc087_b_02");
    let input = r"5
    1
    0
    150";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc087_b_03() {
    let testdir = TestDir::new("abc087_b", "abc087_b_03");
    let input = r"30
    40
    50
    6000";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "213\n");
    assert!(output.stderr_str().is_empty());
}
