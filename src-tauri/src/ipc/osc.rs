use crate::AppData;
use crate::hrm::{HeartRateService};
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
pub async fn hrm_svc_start(app: AppHandle, state: State<'_, AsyncMutex<AppData>>) -> Result<(), &'static str> {
    let mut st = state.lock().await;

    if st.hrm_service.is_some() {
        return Ok(())
    }

    let mut hrm = HeartRateService::new();
    let app_hr = app.clone();
    let app_connect = app.clone();
    let app_disconnect = app.clone();

    hrm.on_heart_rate(move |hr| {
        app_hr.emit("birdpro://hrm/update", hr).ok();

        let app_cb_thread = app_hr.clone();
        tauri::async_runtime::spawn(async move {
            let state = app_cb_thread.state::<AsyncMutex<AppData>>();
            let st = state.lock().await;

            if let Some(osc) = &st.vrc_osc {
                // let param_connected = st.config["heartrate.customConnectedParam"].as_str().unwrap_or("/avatar/parameters/hr_connected");
                let param_percent = st.config["heartrate.customPercentParam"].as_str().unwrap_or("/avatar/parameters/hr_percent");
                let max_hr = st.config["heartrate.customMaxHeartrate"].as_f64().unwrap_or(200.0);

                let msg = OscMessage {
                    addr: param_percent.to_string(),
                    args: vec![OscType::Float(hr as f32 / max_hr as f32)],
                };
                if let Err(e) = osc.send(OscPacket::Message(msg), "VRChat-Client-*").await {
                    log::error!("osc send error: {e}");
                }
            }
        });
    });

    hrm.on_connect(move || {
        app_connect.emit("birdpro://hrm/connected", ()).ok();
    });

    hrm.on_disconnect(move || {
        app_disconnect.emit("birdpro://hrm/disconnected", ()).ok();
    });

    let widget_id = st.config["heartrate.widgetId"].as_str();
    if widget_id.is_none() {
        return Err("Widget ID must be specified");
    }

    let status = hrm.start(widget_id.unwrap().to_string()).await?;

    // emit starting message
    app.emit("birdpro://hrm/connecting", ()).ok();

    st.hrm_service = Some(hrm);

    Ok(status)
}

#[tauri::command]
pub async fn hrm_svc_stop(_app: AppHandle, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()>  {
    let mut st = state.lock().await;

    if st.hrm_service.is_some() {
        st.hrm_service.as_mut().unwrap().stop();
        st.hrm_service = None;
    }

    Ok(())
}

#[tauri::command]
pub async fn hrm_svc_status(state: State<'_, AsyncMutex<AppData>>) -> Result<bool, String>  {
    let st = state.lock().await;

    if st.hrm_service.is_some() {
        return Ok(true)
    } else {
        return Ok(false)
    }
}
