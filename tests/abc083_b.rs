use cli_test_dir::*;

#[test]
fn abc083_b_01() {
    let testdir = TestDir::new("abc083_b", "abc083_b_01");
    let input = r"20 2 5";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "84\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc083_b_02() {
    let testdir = TestDir::new("abc083_b", "abc083_b_02");
    let input = r"10 1 2";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "13\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc083_b_03() {
    let testdir = TestDir::new("abc083_b", "abc083_b_03");
    let input = r"100 4 16";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4554\n");
    assert!(output.stderr_str().is_empty());
}
