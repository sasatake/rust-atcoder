use cli_test_dir::*;

#[test]
fn abc081_a_01() {
    let testdir = TestDir::new("abc081_a", "abc081_a_01");
    let input = r"101";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc081_a_02() {
    let testdir = TestDir::new("abc081_a", "abc081_a_02");
    let input = r"000";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}
