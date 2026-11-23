use serde::Serialize;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub ok: bool,
}

/// Connectivity smoke test for the frontend/backend IPC boundary.
#[tauri::command]
pub fn ping() -> Result<PingResponse, AppError> {
    Ok(PingResponse { ok: true })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_returns_ok_payload() {
        let response = ping().unwrap();
        assert!(response.ok);
        assert_eq!(
            serde_json::to_value(&response).unwrap(),
            serde_json::json!({ "ok": true })
        );
    }

    #[test]
    fn command_error_uses_app_error_contract() {
        let result: Result<PingResponse, AppError> = Err(AppError::new(
            crate::error::ErrorCode::Internal,
            "test failure",
        ));
        let err = result.err().unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({
                "code": "internal",
                "message": "test failure",
                "details": null
            })
        );
    }
}
