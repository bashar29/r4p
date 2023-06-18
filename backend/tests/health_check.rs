use actix_web::{http::header::ContentType, test, web, App};

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app();

    let req = test::TestRequest::get().uri("/health_check").to_request();
    // Act
    let resp = test::call_service(&app, req).await;
    // Assert
    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

fn spawn_app() {
    let server = backend::run().expect("Failed to bind address");
    let _ = tokio::spawn(server)
}
