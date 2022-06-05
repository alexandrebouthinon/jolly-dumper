use assert_cli::Assert;

#[test]
fn info_log() {
  Assert::main_binary()
    .with_args(&["-vv"])
    .stderr()
    .contains("Running jolly dumper")
    .unwrap();
}
