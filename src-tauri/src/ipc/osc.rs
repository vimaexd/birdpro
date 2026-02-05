use crate::AppData;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex as AsyncMutex;
use vrchat_osc::{ServiceType, VRChatOSC,};
use vrchat_osc::rosc::{OscMessage, OscPacket, OscType};

#[tauri::command]
pub async fn osc_start(
    app: AppHandle,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let mut state = state.lock().await;
    let osc = VRChatOSC::new(None).await.unwrap();

    osc.on_connect(move |svc| {
        if let ServiceType::Osc(str, addr) = svc {
            log::info!("connected to OSC ({} on port {})", str, addr);
            app.emit("osc-connected", "").unwrap();
        }
    }).await;

    state.vrc_osc = Some(osc);
    Ok(())
}

#[tauri::command]
pub async fn osc_typing_indicator(
    typing: bool,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let state = state.lock().await;

    if state.vrc_osc.is_some() {
        let msg = OscMessage {
            addr: "/chatbox/typing".to_string(),
            args: vec![
                OscType::Bool(typing) // send immediately
            ]
        };

        state.vrc_osc.as_ref().unwrap()
            .send(OscPacket::Message(msg), "VRChat-Client-*")
            .await.unwrap();
    }

    Ok(())
}
