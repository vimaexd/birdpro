use futures_util::{StreamExt};
use serde_json::Value;
use std::sync::Arc;
use std::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct HeartRateService {
    on_heart_rate: Option<Arc<dyn Fn(u32) + Send + Sync + 'static>>,
    on_connect: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    on_disconnect: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    task: Option<tauri::async_runtime::JoinHandle<()>>,

    connected: Arc<Mutex<bool>>,
}

static PULSOID_RPC_URL: &'static str = "https://pulsoid.net/v1/api/public/rpc";

impl HeartRateService {
    async fn get_ws_url(widget_id: &str) -> Option<String> {
        let client = reqwest::Client::new();

        let body = serde_json::json!({
            "id": "1",
            "jsonrpc": "2.0",
            "method": "getWidget",
            "params": {
                "widgetId": widget_id
            }
        });

        let _req = client
            .post(PULSOID_RPC_URL.to_owned())
            .json(&body)
            .send()
            .await
            .ok()?;

        let json = _req.json::<Value>().await.ok()?;
        let url = json["result"]["ramielUrl"].as_str().map(|s| s.to_string());

        url
    }

    pub fn new() -> Self {
        Self {
            on_heart_rate: None,
            on_connect: None,
            on_disconnect: None,
            task: None,
            connected: Arc::new(Mutex::new(false)),
        }
    }

    pub fn on_heart_rate(&mut self, f: impl Fn(u32) + Send + Sync + 'static) -> &mut Self {
        self.on_heart_rate = Some(Arc::new(f));
        self
    }

    pub fn on_connect(&mut self, f: impl Fn() + Send + Sync + 'static) -> &mut Self {
        self.on_connect = Some(Arc::new(f));
        self
    }

    pub fn on_disconnect(&mut self, f: impl Fn() + Send + Sync + 'static) -> &mut Self {
        self.on_disconnect = Some(Arc::new(f));
        self
    }

    pub fn start(&mut self, widget_id: String) {
        let connected = Arc::clone(&self.connected);
        let on_discon_callback = self.on_disconnect.as_ref().map(Arc::clone);
        let on_con_callback = self.on_connect.as_ref().map(Arc::clone);
        let on_hr_callback = self.on_heart_rate.as_ref().map(Arc::clone);

        let handle = tauri::async_runtime::spawn(async move {
            let ws_url = HeartRateService::get_ws_url(&widget_id).await.unwrap();
            let (mut socket, _) = connect_async(ws_url).await.unwrap();
            log::info!("websocket connected");

            let mut locally_connected = false;

            while let Some(msg) = socket.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            if let Some(bpm) = json["data"]["heartRate"].as_u64() {
                                if !locally_connected {
                                    if let Some(cb) = &on_con_callback {
                                        cb();
                                    }
                                    locally_connected = true;
                                    *connected.lock().unwrap() = true; // sync back
                                }
                                if let Some(cb) = &on_hr_callback {
                                    cb(bpm as u32);
                                }
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("hrm websocket error: {e}");
                        break;
                    }
                }
            }
            log::info!("websocket disconnected");
            if let Some(cb) = &on_discon_callback {
                cb();
            }
        });

        self.task = Some(handle);
    }

    pub fn stop(&mut self) {
        let on_discon_callback = self.on_disconnect.clone();

        if let Some(task) = self.task.take() {
            task.abort();
            if let Some(cb) = &on_discon_callback {
                cb();
            }
            log::info!("hrm socket disconnected");
        }
    }
}
