use crate::helpers;

#[actix_web::test]
async fn create_account_returns_200_for_valid_data() {
    // Arrange
    let test_app = helpers::spawn_app().await;
    let first_valid_creation_body = serde_json::json!({
        "company_name" : "The company corp.",
        "admin_lastname" : "Johnson",
        "admin_firstname" : "Bill",
        "admin_email" : "bill.johnson@gmail.com"
    });
    let second_valid_creation_body = serde_json::json!({
        "company_name" : "The company corp.",
        "admin_lastname" : "Johnson",
        "admin_firstname" : "",
        "admin_email" : "bill.johnson@gmail.com"
    });
    let third_valid_creation_body = serde_json::json!({
        "company_name" : "The company corp.",
        "admin_lastname" : "Johnson",
        "admin_email" : "bill.johnson@gmail.com"
    });

    let test_cases = vec![
        first_valid_creation_body,
        second_valid_creation_body,
        third_valid_creation_body,
    ];

    for test_case in test_cases {
        // Act
        let response = test_app.post_account(&test_case).await;
        // Assert
        assert_eq!(200, response.status().as_u16());
    }
}

#[actix_web::test]
async fn create_account_returns_400_for_invalid_data() {
    // Arrange
    let test_app = helpers::spawn_app().await;

    let first_incomplete_body = (
        serde_json::json!({
            "company_name" : "",
            "admin_lastname" : "Johnson",
            "admin_firstname" : "Bill",
            "admin_email" : "bill.johnson@gmail.com"
        }),
        "mandatory data equals ''",
    );
    let second_incomplete_blody = (
        serde_json::json!({
            "company_name" : "The company corp.",
            "admin_firstname" : "Bill",
            "admin_email" : "bill.johnson@gmail.com"
        }),
        "missing mandatory data",
    );

    let test_cases = vec![first_incomplete_body, second_incomplete_blody];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = test_app.post_account(&invalid_body).await;
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

async fn _create_account_persist_the_new_account() {}

async fn _create_account_create_admin_user_for_the_account() {}
