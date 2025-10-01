use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Form, Json, Router,
};
use axum_template::{engine::Engine, RenderHtml};
use chrono::{DateTime, Utc};
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use timestamp_verify::{sign, verify, VerificationError};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    template_engine: Engine<Environment<'static>>,
}

#[tokio::main]
async fn main() {
    let mut jinja = Environment::new();
    jinja.set_loader(minijinja::path_loader("templates"));

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(get_home))
        .route("/token", get(get_token))
        .route("/verify", get(get_verify).post(post_verify))
        .route("/how-it-works", get(get_how_it_works))
        .with_state(AppState {
            template_engine: Engine::from(jinja),
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn get_home(State(state): State<AppState>) -> Response {
    RenderHtml("index.jinja", state.template_engine, ()).into_response()
}

#[axum::debug_handler]
async fn get_token() -> Json<ValidatedTimestamp> {
    Json(ValidatedTimestamp::get())
}

#[axum::debug_handler]
async fn get_verify(State(state): State<AppState>) -> Response {
    RenderHtml("verify.jinja", state.template_engine, ()).into_response()
}

#[axum::debug_handler]
async fn post_verify(State(state): State<AppState>, Form(payload): Form<VerifyInput>) -> Response {
    let output: VerifyOutput = payload.into();
    RenderHtml("verify.jinja", state.template_engine, output).into_response()
}

#[axum::debug_handler]
async fn get_how_it_works(State(state): State<AppState>) -> Response {
    RenderHtml("how-it-works.jinja", state.template_engine, ()).into_response()
}

#[derive(Serialize, Deserialize, Debug)]
struct VerifyInput {
    timestamp: String,
    token: String,
}

#[derive(Serialize)]
struct VerifyOutput {
    input: VerifyInput,
    result: Result<String, String>,
}

impl From<VerifyInput> for VerifyOutput {
    fn from(input: VerifyInput) -> Self {
        Self {
            result: input
                .timestamp
                .parse::<i64>()
                .map_err(|_| "Timestamp is not valid".to_owned())
                .and_then(|v| verify(v, &input.token).map_err(|e: VerificationError| e.to_string()))
                .and_then(|v| {
                    DateTime::from_timestamp(v, 0)
                        .ok_or_else(|| "Timestamp is not valid".to_owned())
                })
                .map(|dt| dt.format("%B %e, %Y %H:%M:%S").to_string()),
            input,
        }
    }
}

#[derive(Serialize)]
struct ValidatedTimestamp {
    time_formatted: String,
    timestamp: i64,
    token: String,
}

impl ValidatedTimestamp {
    fn get() -> Self {
        let date = Utc::now();

        let mac = sign(date.timestamp());

        Self {
            time_formatted: format!("{}", date.format("%B %e, %Y %H:%M:%S")),
            timestamp: date.timestamp(),
            token: mac,
        }
    }
}
