use cli_test_dir::*;

#[test]
fn abc085_c_01() {
    let testdir = TestDir::new("abc085_c", "abc085_c_01");
    let input = r"9 45000";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4 0 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc085_c_02() {
    let testdir = TestDir::new("abc085_c", "abc085_c_02");
    let input = r"20 196000";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1 -1 -1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc085_c_03() {
    let testdir = TestDir::new("abc085_c", "abc085_c_03");
    let input = r"1000 1234000";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "26 0 974\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc085_c_04() {
    let testdir = TestDir::new("abc085_c", "abc085_c_04");
    let input = r"2000 20000000";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2000 0 0\n");
    assert!(output.stderr_str().is_empty());
}
