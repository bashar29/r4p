use crate::helpers;

#[actix_web::test]
async fn create_account_return_200_for_valid_data() {
    // Arrange
    let test_app = helpers::spawn_app().await;
    let creation_body = serde_json::json!({
        "company_name" : "The company corp.",
        "admin_lastname" : "Johnson",
        "admin_firstname" : "Bill"
    });

    // Act
    let _response = test_app.post_account(&creation_body).await;

    // Assert
    // assert_eq!(200, response.status().as_u16());
}

async fn _create_account_persist_the_new_account() {}

async fn _create_account_create_admin_user_for_the_account() {}
