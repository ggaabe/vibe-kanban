use axum::{routing::get, Json, Router};
use tokio::process::Command;

use crate::{app_state::AppState, models::ApiResponse};

pub fn opencode_router() -> Router<AppState> {
    Router::new().route("/opencode/models", get(get_models))
}

pub async fn get_models() -> Json<ApiResponse<Vec<String>>> {
    let output = Command::new("opencode").arg("models").output().await;
    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let models = stdout
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
            Json(ApiResponse::success(models))
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            Json(ApiResponse::error(&format!("Failed to list models: {}", err)))
        }
        Err(e) => Json(ApiResponse::error(&format!("Failed to run opencode: {}", e))),
    }
}
