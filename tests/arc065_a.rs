use cli_test_dir::*;

#[test]
fn arc065_a_01() {
    let testdir = TestDir::new("arc065_a", "arc065_a_01");
    let input = r"erasedream";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn arc065_a_02() {
    let testdir = TestDir::new("arc065_a", "arc065_a_02");
    let input = r"dreameraser";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn arc065_a_03() {
    let testdir = TestDir::new("arc065_a", "arc065_a_03");
    let input = r"dreamerer";
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "NO\n");
    assert!(output.stderr_str().is_empty());
}
