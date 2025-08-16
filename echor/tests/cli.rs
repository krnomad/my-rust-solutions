use std::{fs};
use anyhow::Result;
use assert_cmd::Command;

fn dies_no_args() -> Result<()> {
    // TODO here
    Ok(())
}

// fn runs(args: &[&str], expected_file: &str) -> Result<()> {
fn runs(args: &[&str], expected_file: &str) -> Result<()> {
    // TODO here too
    let expected = fs::read_to_string("tests/expectd/hello2.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"]).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn run_batch() {
    let outfile = "tests/expectd/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    let expected = fs::read_to_string("tests/expectd/hello1.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> Result<()> {
    let expected = fs::read_to_string("tests/expectd/hello2.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"]).assert().success().stdout(expected);
    Ok(())
}

