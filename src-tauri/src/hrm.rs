use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{future, pin_mut, StreamExt};
use std::sync::Arc;

pub struct PulsoidService {
    on_heart_rate: Option<Arc<dyn Fn(u32) + Send + Sync + 'static>>,
    task: Option<tauri::async_runtime::JoinHandle<()>>,
}

static PULSOID_RPC_URL: &'static str = "https://pulsoid.net/v1/api/public/rpc";

impl PulsoidService {
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
        Self { on_heart_rate: None, task: None }
    }

    pub fn on_heart_rate(&mut self, f: impl Fn(u32) + Send + Sync + 'static) -> &mut Self {
        self.on_heart_rate = Some(Arc::new(f));
        self
    }

    pub fn start(&mut self) {
        let callback = self.on_heart_rate.as_ref().map(Arc::clone);

        let handle = tauri::async_runtime::spawn(async move {
            let url = "wss://ramiel2.pulsoid.net/listen/7f20184a-5fcf-11e6-8b77-86f30ca893d3".to_string();
            let (mut socket, _) = connect_async(url).await.unwrap();

            while let Some(msg) = socket.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            if let Some(bpm) = json["data"]["heartRate"].as_u64() {
                                if let Some(cb) = &callback {
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
        });

        self.task = Some(handle);
    }

    pub fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
            log::info!("hrm socket disconnected");
        }
    }
}
