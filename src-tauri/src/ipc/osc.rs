use crate::AppData;
use crate::hrm::PulsoidService;
use log::info;
use tauri::{AppHandle, Emitter, State, Manager};
use tokio::sync::Mutex as AsyncMutex;
use vrchat_osc::rosc::{OscMessage, OscPacket, OscType};
use vrchat_osc::{ServiceType, VRChatOSC};
use std::net::{IpAddr, Ipv4Addr};

#[tauri::command]
pub async fn osc_start(app: AppHandle, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut state = state.lock().await;

    // spinning up instances of VRChatOSC is VERY expensive
    // and causes insane CPU usage
    if state.vrc_osc.is_some() {
        return Ok(());
    }

    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let osc = VRChatOSC::new(Some(localhost)).await.unwrap();

    info!("starting OSC");

    osc.on_connect(move |svc| {
        if let ServiceType::Osc(str, addr) = svc {
            log::info!("discovered OSC service ({} on port {})", str, addr);
            app.emit("osc-connected", "").unwrap();
        }
    })
    .await;

    state.vrc_osc = Some(osc);
    Ok(())
}

#[tauri::command]
pub async fn osc_stop(state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut state = state.lock().await;
    if state.vrc_osc.is_some() {
        info!("stopping OSC");
        state.vrc_osc.as_ref().unwrap().shutdown().await.unwrap();
        state.vrc_osc = None;
    }
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
                OscType::Bool(typing), // send immediately
            ],
        };

        log::info!("sending OSC typing indicator");

        state
            .vrc_osc
            .as_ref()
            .unwrap()
            .send(OscPacket::Message(msg), "VRChat-Client-*")
            .await
            .unwrap();
    }

    Ok(())
}

#[tauri::command]
pub async fn hrm_svc_start(app: AppHandle, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()>  {
    let mut st = state.lock().await;

    if st.pulsoid_service.is_some() {
        return Ok(())
    }

    let mut pulsoid = PulsoidService::new();
    pulsoid.on_heart_rate(move |hr| {
        let ah = app.clone();
        log::info!("heart rate: {hr}");
        app.emit("birdpro://hrm/update", hr).ok();

        tauri::async_runtime::spawn(async move {
            let state = ah.state::<AsyncMutex<AppData>>();
            let st = state.lock().await;

            if let Some(osc) = &st.vrc_osc {
                let msg = OscMessage {
                    addr: "/avatar/parameters/hr_percent".to_string(),
                    args: vec![OscType::Float(hr as f32 / 200.0)],
                };
                if let Err(e) = osc.send(OscPacket::Message(msg), "VRChat-Client-*").await {
                    log::error!("osc send error: {e}");
                }
            }
        });
    });
    pulsoid.start();

    st.pulsoid_service = Some(pulsoid);

    Ok(())
}

#[tauri::command]
pub async fn hrm_svc_stop(_app: AppHandle, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()>  {
    let mut st = state.lock().await;

    if st.pulsoid_service.is_some() {
        st.pulsoid_service.as_mut().unwrap().stop();
        st.pulsoid_service = None;
    }

    Ok(())
}
