//! Uption integration tests.
use assert_cmd::Command;
use std::str;

#[test]
fn test_uption_start_with_empty_config() {
    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .env("UPTION_COLLECTORS_PING_ENABLED", "false")
        .env("UPTION_COLLECTORS_HTTP_ENABLED", "false")
        .timeout(std::time::Duration::from_secs(1))
        .output()
        .unwrap();
    output.status.success();
    assert!(
        str::from_utf8(&output.stdout)
            .unwrap()
            .contains("No collectors configured!"),
        "Output did not contain text 'No collectors configured!'"
    );
}
