// tests/integration_test.rs
use axum_test_sample::main::routes;
use serde_json::Value;
use tokio::time::Duration;

#[tokio::test]
async fn test_hello_endpoint() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let app = routes();

    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap()
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let url = format!("http://{}/hello", addr);
    let resp = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();

    let json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["message"], "Hello, World!");

    drop(server_handle);
}
