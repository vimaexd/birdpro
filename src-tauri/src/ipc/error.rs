use crate::provider::TTSBackendError;

#[tauri::command]
pub fn get_error_text(error_code: TTSBackendError) -> String {
    format!("{}", error_code)
}
