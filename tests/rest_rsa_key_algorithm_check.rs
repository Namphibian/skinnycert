mod common;
use common::spawn_app;



#[tokio::test]
async fn get_rsa_key_algorithm_test() {
    // --- Arrange ---
    // Start an ephemeral instance of the server and retrieve its base URL.
    let address = spawn_app().await;
    println!("Test server listening on {}", address);

    // Initialize an asynchronous HTTP client.
    let client = reqwest::Client::new();

    // --- Act ---
    // Send a GET request to the `/health` endpoint.
    let response = client
        .get(&format!("{}/keys/rsa", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    let status = response.status();
    let content_length = response.content_length().unwrap_or(0);

    // Now consume the response to get the text body
    let response_text = response.text().await.unwrap();
    print!("Response: {}", response_text);
    // --- Assert ---
    // The endpoint should return a successful 2xx response.
    assert!(
        status.is_success(),
        "Health check did not return a successful status code"
    );

    // The body should not be empty — it should contain JSON data.
    assert!(
        content_length > 0,
        "Health check returned an empty response body"
    );
}
