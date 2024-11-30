// tests/auth_tests.rs

#[cfg(test)]
mod tests {
    use crate::tests::web::Bytes;

    use actix_web::dev::ServiceResponse;
    use actix_web::{test, web, App};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::{PgConnection, RunQueryDsl};
    use dotenvy::from_filename;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde_json::Value;

    use comu::modules::auth::handler::{login, register};
    use comu::modules::auth::jwt::Claims;
    use comu::modules::auth::middleware::JwtMiddleware;
    use comu::utils::db::init_pool;

    /// Setup the test environment
    fn setup() -> Pool<ConnectionManager<PgConnection>> {
        // Load environment variables
        from_filename(".env.test").unwrap();

        // Initialize the database
        let pool = init_pool(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

        // Truncate the users table
        let mut conn = pool.get().unwrap();

        // Truncate the users table
        diesel::sql_query("TRUNCATE TABLE users CASCADE")
            .execute(&mut conn)
            .unwrap();

        // Return the pool
        pool
    }

    /// Teardown the test environment
    fn teardown(pool: &Pool<ConnectionManager<PgConnection>>) {
        // Initialize the database
        let mut conn = pool.get().unwrap();

        // Truncate the users table
        diesel::sql_query("TRUNCATE TABLE users CASCADE")
            .execute(&mut conn)
            .unwrap();
    }

    /// Test register user success
    #[actix_web::test]
    async fn test_register_user_success() {
        // Setup the database
        let pool = setup();

        // Initialize the app
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/register", web::post().to(register)),
        )
        .await;

        // Request body
        let req_body = r#"{
            "email": "test@example.com",
            "username": "test_user",
            "password": "securepassword"
        }"#;

        // Create a request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_payload(req_body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        // Call the service
        let resp = test::call_service(&app, req).await;

        // Check response status
        assert_eq!(resp.status(), 200);

        // Cleanup
        teardown(&pool);
    }

    //// Test register user duplicate email
    #[actix_web::test]
    async fn test_register_user_duplicate_email() {
        // Setup the database
        let pool = setup();

        // Initialize the app
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/register", web::post().to(register)),
        )
        .await;

        // First user registration
        let req_body = r#"
        {
            "email": "test@example.com",
            "username": "test_user",
            "password": "securepassword"
        }
        "#;

        println!("Request Body: {}", req_body);

        // Create a request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_payload(req_body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        // Call the service
        let _ = test::call_service(&app, req).await;

        // Second user registration with the same email
        let req_body = r#"
        {
            "email": "test@example.com",
            "username": "another_user",
            "password": "anotherpassword"
        }
        "#;

        println!("Request Body: {}", req_body);

        // Create a request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_payload(req_body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        // Call the service
        let resp: ServiceResponse = test::call_service(&app, req).await;

        // Check response status
        assert_eq!(resp.status(), 400);

        // Check response body
        let resp_body: Bytes = test::read_body(resp).await;
        let json_resp: Value = serde_json::from_slice(&resp_body).unwrap();
        assert_eq!(json_resp["message"], "Email already exists");

        // Cleanup
        teardown(&pool);
    }

    /// Test login user success
    #[actix_web::test]
    async fn test_login_user_success() {
        // Setup the database
        let pool = setup();

        // Initialize the app
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/register", web::post().to(register))
                .route("/login", web::post().to(login)),
        )
        .await;

        // Register user
        let req_body = r#"
        {
            "email": "test@example.com",
            "username": "test_user",
            "password": "securepassword"
        }
        "#;

        // Create a request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_payload(req_body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        // Call the service
        let _ = test::call_service(&app, req).await;

        // Login user request
        let req_body = r#"
        {
            "email": "test@example.com",
            "password": "securepassword"
        }
        "#;

        // Create a request
        let req = test::TestRequest::post()
            .uri("/login")
            .set_payload(req_body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        // Call the service
        let resp = test::call_service(&app, req).await;

        // Check response status
        assert_eq!(resp.status(), 200);

        // Check response body
        let resp_body = test::read_body(resp).await;
        let json_resp: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
        assert!(json_resp["token"].as_str().is_some());

        // Cleanup
        teardown(&pool);
    }

    /// Test login user failure
    #[actix_web::test]
    async fn test_login_user_failure() {
        // Setup the database
        let pool = setup();

        // Initialize the app
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/register", web::post().to(register))
                .route("/login", web::post().to(login)),
        )
        .await;

        // Register user
        let req_body = r#"
        {
            "email": "test@example.com",
            "username": "test_user",
            "password": "securepassword"
        }
        "#;

        // Create a request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_payload(req_body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        // Call the service
        let _ = test::call_service(&app, req).await;

        // Login user request
        let req_body = r#"
        {
            "email": "test@example.com",
            "password": "wrongpassword"
        }
        "#;

        // Create a request
        let req = test::TestRequest::post()
            .uri("/login")
            .set_payload(req_body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        // Call the service
        let resp = test::call_service(&app, req).await;

        // Check response status
        assert_eq!(resp.status(), 401);

        // Cleanup
        teardown(&pool);
    }

    /// Generate a test JWT token
    fn generate_test_token() -> String {
        let claims = Claims {
            sub: "user_id".to_string(),
            exp: 2000000000, // 유효기간: 적절히 설정
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("your_test_secret".as_ref()),
        )
        .expect("Failed to generate token")
    }

    /// Test JWT middleware success
    #[actix_web::test]
    async fn test_jwt_middleware_success() {
        std::env::set_var("JWT_SECRET", "your_test_secret");

        // Initialize the app
        let app = test::init_service(
            App::new()
                .wrap(JwtMiddleware)
                .route("/protected", web::get().to(|| async { "Access granted" })),
        )
        .await;

        // Test JWT token
        let token = generate_test_token();

        // Create a request
        let req = test::TestRequest::get()
            .uri("/protected")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        // Call the service
        let resp = test::call_service(&app, req).await;

        // Validate the response status
        assert_eq!(resp.status(), 200);
    }

    /// Test JWT middleware failure
    #[actix_web::test]
    async fn test_jwt_middleware_failure() {
        std::env::set_var("JWT_SECRET", "your_test_secret");

        // Initialize the app
        let app = test::init_service(
            App::new()
                .wrap(JwtMiddleware)
                .route("/protected", web::get().to(|| async { "Access granted" })),
        )
        .await;

        // Create a request
        let req = test::TestRequest::get()
            .uri("/protected")
            .insert_header(("Authorization", "Bearer invalid_token"))
            .to_request();

        // Call the service
        let resp = test::call_service(&app, req).await;

        // Print the response status
        println!("Response Status: {}", resp.status());

        // Validate the response status
        assert_eq!(resp.status(), 401);
    }
}
