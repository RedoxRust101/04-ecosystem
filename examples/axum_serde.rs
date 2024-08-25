use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
  extract::State,
  routing::{get, patch},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
  fmt::{self, format::FmtSpan},
  layer::SubscriberExt,
  util::SubscriberInitExt,
  Layer,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
struct User {
  name: String,
  age: u8,
  skills: Vec<String>,
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
  age: Option<u8>,
  skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
  let console =
    fmt::Layer::new().with_span_events(FmtSpan::CLOSE).pretty().with_filter(LevelFilter::DEBUG);

  tracing_subscriber::registry().with(console).init();

  let user =
    User { name: "John".to_string(), age: 30, skills: vec!["rust".to_string(), "go".to_string()] };
  let user = Arc::new(Mutex::new(user));

  let addr = "0.0.0.0:8080";
  let listener = TcpListener::bind(addr).await?;
  info!("listening on {}", addr);

  let app =
    Router::new().route("/", get(user_handler)).route("/", patch(unpdate_user)).with_state(user);
  axum::serve(listener, app).await?;

  Ok(())
}

#[instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
  (*user.lock().unwrap()).clone().into()
}

#[instrument]
async fn unpdate_user(
  State(user): State<Arc<Mutex<User>>>,
  Json(user_update): Json<UserUpdate>,
) -> Json<User> {
  let mut user = user.lock().unwrap();
  if let Some(age) = user_update.age {
    user.age = age;
  }
  if let Some(skills) = user_update.skills {
    user.skills = skills;
  }
  (*user).clone().into()
}
