// use std::process::Command;

use assert_cmd::Command;

#[test]
fn work() {
    // let mut cmd = Command::new("hello");
    // let res = cmd.output();
    // assert!(res.is_ok());
    // assert!(true);

    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}