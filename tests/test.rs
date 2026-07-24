mod common;

use crate::common::setup::*;
use test_log::test;
use zero2prod::User;

#[test(tokio::test)]
async fn get_healthtest() {
    let (addr, _guard) = start_server().await.unwrap();
    let client = get_client();
    let url = format!("http://{}{}", addr, "/health");
    let res = client.get(url).send().await.unwrap();

    assert!(res.status().is_success());
    println!("res: {}", res.text_with_charset("utf-8").await.unwrap());
}

#[test(tokio::test)]
async fn post_sub_ok() {
    let (addr, _guard) = start_server().await.unwrap();
    let client = get_client();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = client
        .post(format!("http://{}{}", addr, "/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(res.status().is_success());

    let res: Vec<User> = client
        .get(format!("http://{}{}", addr, "/subscriptions"))
        .send()
        .await
        .expect("Failed to execute request.")
        .json()
        .await
        .expect("failed to deserialize response to json");

    assert_eq!(res[0].name, "le guin");
    assert_eq!(res[0].email, "ursula_le_guin@gmail.com");
}

#[test(tokio::test)]
async fn post_sub_err() {
    let (addr, _guard) = start_server().await.unwrap();
    let client = get_client();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("http://{}{}", addr, "/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
