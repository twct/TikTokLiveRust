use crate::core::live_client::TikTokLiveClient;
use crate::core::live_client_events::{TikTokEventHandler, TikTokLiveEventObserver};
use crate::core::live_client_http::TikTokLiveHttpClient;
use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::core::live_client_websocket::TikTokLiveWebsocketClient;
use crate::data::create_default_settings;
use crate::data::live_common::{TikTokLiveInfo, TikTokLiveSettings};
use crate::generated::events::TikTokLiveEvent;
use crate::http::http_request_builder::HttpRequestFactory;
use tokio::sync::mpsc;

pub struct TikTokLiveBuilder {
    settings: TikTokLiveSettings,
    pub(crate) event_observer: TikTokLiveEventObserver,
    event_sender: mpsc::Sender<TikTokLiveEvent>,
}

impl TikTokLiveBuilder {
    ///
    ///  # Create new tiktok live builder
    ///
    ///  ### user_name - name of tiktok user that can be found in the live link
    ///
    pub fn new(user_name: &str, event_sender: mpsc::Sender<TikTokLiveEvent>) -> Self {
        Self {
            settings: create_default_settings(user_name),
            event_observer: TikTokLiveEventObserver::new(),
            event_sender,
        }
    }

    ///
    ///  # Configure live connection settings
    ///
    ///
    pub fn configure<F>(&mut self, on_configure: F) -> &mut Self
    where
        F: FnOnce(&mut TikTokLiveSettings),
    {
        on_configure(&mut self.settings);
        self
    }

    ///
    ///  # Invoked every time new event is coming from tiktok
    ///
    ///    ## client - instance of TikTokLiveClient
    ///    ## event  - invoked event
    ///  ```
    ///
    pub fn on_event(&mut self, on_event: TikTokEventHandler) -> &mut Self {
        self.event_observer.subscribe(on_event);
        self
    }

    ///
    /// Returns new instance of TikTokLiveClient
    ///
    pub fn build(&self) -> TikTokLiveClient {
        let settings = &self.settings;
        let observer = self.event_observer.clone();
        let mapper = TikTokLiveMessageMapper {};
        let websocket_client = TikTokLiveWebsocketClient::new(mapper, self.event_sender.clone());
        let http_factory = HttpRequestFactory {
            settings: settings.clone(),
        };
        let http_client = TikTokLiveHttpClient {
            settings: settings.clone(),
            factory: http_factory,
        };

        return TikTokLiveClient::new(
            settings.clone(),
            http_client,
            observer,
            websocket_client,
            TikTokLiveInfo::default(),
            self.event_sender.clone(),
        );
    }
}
