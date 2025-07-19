use axum::{routing::get, Json, Router};
use axum::http::StatusCode;
use axum::response::Json as ResponseJson;
use crate::{app_state::AppState, models::ApiResponse, utils::shell::get_shell_command};

pub async fn list_models() -> Result<ResponseJson<ApiResponse<Vec<String>>>, StatusCode> {
    use tokio::process::Command;

    let (shell_cmd, shell_arg) = get_shell_command();
    let output = Command::new(shell_cmd)
        .arg(shell_arg)
        .arg("npx -y opencode-ai@latest models")
        .output()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !output.status.success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let models = stdout
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    Ok(ResponseJson(ApiResponse {
        success: true,
        data: Some(models),
        message: None,
    }))
}

pub fn opencode_router() -> Router<AppState> {
    Router::new().route("/opencode/models", get(list_models))
}
