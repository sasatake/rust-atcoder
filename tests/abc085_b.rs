use cli_test_dir::*;

#[test]
fn abc085_b_01() {
    let testdir = TestDir::new("abc085_b", "abc085_b_01");
    let input = r"4
    10
    8
    8
    6";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc085_b_02() {
    let testdir = TestDir::new("abc085_b", "abc085_b_02");
    let input = r"3
    15
    15
    15";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc085_b_03() {
    let testdir = TestDir::new("abc085_b", "abc085_b_03");
    let input = r"7
    50
    30
    50
    100
    50
    80
    30";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}
