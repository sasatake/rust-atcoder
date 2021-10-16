use cli_test_dir::*;

#[test]
fn abc086_a_01() {
    let testdir = TestDir::new("abc086_a", "abc086_a_01");
    let input = r"3 4";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Even\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc086_a_02() {
    let testdir = TestDir::new("abc086_a", "abc086_a_02");
    let input = r"1 21";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Odd\n");
    assert!(output.stderr_str().is_empty());
}
