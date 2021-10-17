use cli_test_dir::*;

#[test]
fn abc081_b() {
    let testdir = TestDir::new("abc081_b", "abc081_b_01");
    let input = r"3
    8 12 40";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc081_b_02() {
    let testdir = TestDir::new("abc081_b", "abc081_b_02");
    let input = r"4
    5 6 8 10";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn abc081_b_03() {
    let testdir = TestDir::new("abc081_b", "abc081_b_03");
    let input = r"6
    382253568 723152896 37802240 379425024 404894720 471526144";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8\n");
    assert!(output.stderr_str().is_empty());
}
