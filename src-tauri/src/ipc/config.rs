use crate::AppData;
use serde_json::Value;
use tauri::State;
use tokio::sync::Mutex as AsyncMutex;

// Updates a copy of the config on the backend so that native functions
// can access the most up to date config without having to
// read from disk constantly
#[tauri::command]
pub async fn update_config(config: Value, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut st = state.lock().await;
    st.config = config;
    Ok(())
}
