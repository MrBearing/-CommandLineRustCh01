use assert_cmd::Command;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    use std::process::Command;
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
    let mut cmd = Command::new("hello");
    let res = cmd.output();
    assert!(res.is_err());
}

#[test]
fn runs2() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn check_correct_output() {
    use assert_cmd::Command;
    let mut cmd = Command::cargo_bin("hello").unwrap();
    // cmd.assert().success().stdout("Hello,world!\n");// failure case . need space.
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    use assert_cmd::Command;
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    use assert_cmd::Command;
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
