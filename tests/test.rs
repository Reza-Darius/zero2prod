use reqwest::StatusCode;

use crate::common::setup::{get_client, start_server};

mod common;

#[tokio::test]
#[test_log::test]
async fn my_test() {
    let addr = start_server().await.unwrap();
    let client = get_client();
    let url = format!("http://localhost:{}{}", addr.port(), "/health");
    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}
