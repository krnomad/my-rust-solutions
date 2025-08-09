use std::process::Command;

#[test]
fn work() {
    let mut cmd = Command::new("hello");
    let res = cmd.output();
    assert!(res.is_ok());
    assert!(true);
}