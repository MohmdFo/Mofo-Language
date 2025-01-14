use std::process::Command;

#[test]
fn test_file_reading() {
    let output = Command::new("./target/debug/mofo-language")
        .arg("examples/hello.mofo")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Hello, World!"));
}
