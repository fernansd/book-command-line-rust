use std::process::Command;

// Test file for Integration tests (outside-in)
#[test]
fn runs() {
    let mut cmd = Command::new("cmd.exe");
    let res = cmd.output();
    assert!(res.is_ok());
}