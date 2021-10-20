use cli_test_dir::*;

#[test]
fn abc088_b_01() {
    let testdir = TestDir::new("abc088_b", "abc088_b_01");
    let input = r"2
    3 1";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc088_b_02() {
    let testdir = TestDir::new("abc088_b", "abc088_b_02");
    let input = r"3
    2 7 4";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc088_b_03() {
    let testdir = TestDir::new("abc088_b", "abc088_b_03");
    let input = r"4
    20 18 2 18";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "18\n");
    assert!(output.stderr_str().is_empty());
}
