use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use serde::Serialize;
use serde_json::{json};

#[derive(Debug)]
enum ApiError {
    NotFound(String),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({
            "error":err_msg
        }));
        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(home))
        .route("/user", get(users))
        .route("/user/{id}", get(get_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to create the server");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status":"ok",
        "message":"server is running good"
    }))
}

async fn home() -> &'static str {
    "Home /"
}

#[derive(Serialize)]
struct User {
    name: String,
}

async fn users() -> Result<Json<Vec<User>>, ApiError> {
    let _users = vec![
        User {
            name: String::from("Pradeep"),
        },
        User {
            name: String::from("Bittu"),
        },
    ];

    Err(ApiError::NotFound("Not found".to_string()))
}

#[derive(Serialize)]
struct UserResponse {
    status: String,
    id: i32,
    user: User,
}

async fn get_user(Path(id): Path<i32>) -> Result<Json<UserResponse>, ApiError> {
    if id > 100 {
        return Err(ApiError::InternalError(
            "Id greeter than 100 not accepted".to_string(),
        ));
    };
    Ok(Json(UserResponse {
        status: "ok".to_string(),
        id,
        user: User {
            name: "pradeep".to_string(),
        },
    }))
}
