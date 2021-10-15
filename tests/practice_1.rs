use cli_test_dir::*;

#[test]
fn practice_1_01() {
    let testdir = TestDir::new("practice_1", "practice_1_01");
    let input = r"1
    2 3
    test";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6 test\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn practice_1_02() {
    let testdir = TestDir::new("practice_1", "practice_1_02");
    let input = r"72
    128 256
    myonmyon";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "456 myonmyon\n");
    assert!(output.stderr_str().is_empty());
}
