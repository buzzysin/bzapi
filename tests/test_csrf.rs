
#[tokio::test]
async fn main() {
    let client = reqwest::Client::new();
    let url = "http://localhost:8000/csrf";
    let response = client.get(url).send().await.unwrap();

    if response.status().is_success() {
        println!("CSRF token retrieved successfully.");
        let csrf_token: String = response.text().await.unwrap();
        println!("CSRF Token: {}", csrf_token);
    } else {
        println!("Failed to retrieve CSRF token.");
    }
}