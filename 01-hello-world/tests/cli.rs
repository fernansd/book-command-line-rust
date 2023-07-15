use assert_cmd::Command;

// Test file for Integration tests (outside-in)
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}