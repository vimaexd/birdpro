use crate::provider::TTSProviderError;

#[tauri::command]
pub fn get_error_text(error_code: TTSProviderError) -> String {
    format!("{}", error_code)
}
