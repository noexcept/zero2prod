//! tests/health_check.rs
#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app");

    let client = reqwest::Client::new();

    let resp = client.get("http://127.0.0.1:8000/health_check").send().await.expect("Failed to send request");
    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await
}