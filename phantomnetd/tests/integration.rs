use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn test_node_submission() {
    let server = thread::spawn(|| {
        Command::new("./target/release/phantomnetd")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to start server");
        thread::sleep(Duration::from_secs(2));
    });

    thread::sleep(Duration::from_secs(2)); // Allow server to spin up

    let body = r#"{ "id": "test1", "data": "testdata", "proof": "valid" }"#;
    let status = Command::new("curl")
        .args(&["-X", "POST", "http://localhost:3000/api/tangle", "-H", "Content-Type: application/json", "-d", body])
        .status()
        .expect("curl failed");

    assert!(status.success());
    let _ = server.join();
}