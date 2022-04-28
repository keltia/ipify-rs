use assert_cmd::Command;

const BIN: &str = "ipify-cli";

#[test]
fn test_empty_args() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();

    cmd.assert().success();
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();

    cmd.arg("-h").assert().success();
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();

    cmd.arg("-V").assert().success();
}

