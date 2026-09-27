use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Deserialize;
use std::{env, sync::Arc, time::SystemTime};
use tower_http::cors::{Any, CorsLayer};

const CUTE_MESSAGE: &str = "Ти прелесть, цьом";

#[derive(Deserialize)]
struct SendCuteRequest {
    email: String,
}

#[derive(Clone)]
struct AppState {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    smtp_user: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    dotenvy::from_filename("../.env").ok();

    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let creds = Credentials::new(smtp_user.clone(), smtp_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .expect("Failed to create SMTP transport")
        .credentials(creds)
        .build();

    let state = Arc::new(AppState { mailer, smtp_user });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::GET])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/send", post(send_cute_mail))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("Server listening on http://0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}

async fn send_cute_mail(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendCuteRequest>,
) -> impl IntoResponse {
    let email = payload.email.trim().to_string();
    if email.is_empty() || !email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Invalid email address" })),
        );
    }

    let plain_body = format!(
        "Привіт!\n\nХтось надіслав вам тепле повідомлення через CuteSender:\n\n\"{}\"\n\nГарного дня!\nНадіслано через CuteSender",
        CUTE_MESSAGE
    );

    let html_body = format!(
        "<div style=\"font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 24px; border-radius: 12px; background: #fff5f5; color: #4a2c2c; max-width: 480px;\">
            <h2 style=\"color: #e27d8e; margin-top: 0;\">Привіт!</h2>
            <p style=\"font-size: 15px; color: #6d4c51;\">Хтось вирішив надіслати вам тепле повідомлення через сервіс CuteSender:</p>
            <div style=\"padding: 16px; background: #ffffff; border-left: 4px solid #e27d8e; border-radius: 6px; font-size: 16px; margin: 16px 0; color: #333333;\">
                {}
            </div>
            <p style=\"font-size: 15px; color: #6d4c51;\">Гарного настрою та чудового дня!</p>
            <hr style=\"border: none; border-top: 1px dashed #f1b7be; margin: 20px 0;\" />
            <small style=\"color: #9c7b80;\">Надіслано через CuteSender</small>
        </div>",
        CUTE_MESSAGE
    );

    let email_message = Message::builder()
        .from(format!("CuteSender <{}>", state.smtp_user).parse().unwrap())
        .to(email.parse().unwrap())
        .subject("CuteSender: нове тепле повідомлення")
        .date(SystemTime::now().into())
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(plain_body),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html_body),
                ),
        );

    let email_message = match email_message {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("Failed to construct email: {err}");
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Failed to construct email" })),
            );
        }
    };

    match state.mailer.send(email_message).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "status": "ok", "message": "Email sent successfully" })),
        ),
        Err(err) => {
            eprintln!("SMTP delivery error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to send email via SMTP" })),
            )
        }
    }
}
