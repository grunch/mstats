use std::collections::{HashMap, HashSet};
use std::time::Duration;

use nostr_sdk::prelude::*;

const WINDOW_LIMIT: usize = 500;
const LOOKBACK_WINDOW_SECS: u64 = 7 * 24 * 60 * 60;
const MAX_EMPTY_WINDOWS: usize = 8;

use crate::config::Config;
use crate::models::NostrEvent;

const RELAY_TIMEOUT: Duration = Duration::from_secs(30);

/// Relay client wrapper for fetching Nostr events.
///
/// v1 connects to a single relay with a timeout-based model.
pub struct RelayClient {
    config: Config,
    client: Client,
}

impl RelayClient {
    pub fn new(config: Config) -> Self {
        let client = Client::default();
        RelayClient { config, client }
    }

    /// Ensure we're connected to the relay.
    async fn ensure_connected(&self) -> Result<(), String> {
        let relay_url =
            Url::parse(&self.config.relay_url).map_err(|e| format!("Invalid relay URL: {}", e))?;

        self.client
            .add_relay(relay_url.clone())
            .await
            .map_err(|e| format!("Failed to add relay: {}", e))?;

        self.client.connect().await;

        self.client.wait_for_connection(RELAY_TIMEOUT).await;

        Ok(())
    }

    async fn disconnect(&self) {
        self.client.disconnect().await;
    }

    /// Fetch kind 8383 events from the relay using explicit time windows.
    pub async fn fetch_kind_8383_events(&self) -> Result<Vec<NostrEvent>, String> {
        self.ensure_connected().await?;
        let result = self.fetch_windowed_events(Kind::Custom(8383)).await;
        self.disconnect().await;
        result
    }

    /// Fetch kind 38383 events from the relay using explicit time windows.
    pub async fn fetch_kind_38383_events(&self) -> Result<Vec<NostrEvent>, String> {
        self.ensure_connected().await?;
        let result = self.fetch_windowed_events(Kind::Custom(38383)).await;
        self.disconnect().await;
        result
    }

    async fn fetch_windowed_events(&self, kind: Kind) -> Result<Vec<NostrEvent>, String> {
        let now = Timestamp::now().as_secs();
        let mut window_end = now;
        let mut by_id: HashMap<String, NostrEvent> = HashMap::new();
        let mut seen_ranges: HashSet<(u64, u64, usize)> = HashSet::new();
        let mut consecutive_empty_windows = 0usize;

        loop {
            let window_start = window_end.saturating_sub(LOOKBACK_WINDOW_SECS);
            let filter = Filter::new()
                .kind(kind)
                .since(Timestamp::from(window_start))
                .until(Timestamp::from(window_end))
                .limit(WINDOW_LIMIT);

            let events = self
                .client
                .fetch_events(filter, RELAY_TIMEOUT)
                .await
                .map_err(|e| format!("Failed to fetch kind {} events: {}", kind.as_u16(), e))?;

            let batch: Vec<NostrEvent> = events.iter().map(nostr_event_to_model).collect();
            let batch_len = batch.len();

            if !seen_ranges.insert((window_start, window_end, batch_len)) {
                break;
            }

            if batch_len == 0 {
                consecutive_empty_windows += 1;
            } else {
                consecutive_empty_windows = 0;
            }

            for event in batch.iter().cloned() {
                by_id.entry(event.id.clone()).or_insert(event);
            }

            if window_start == 0 {
                break;
            }

            if consecutive_empty_windows >= MAX_EMPTY_WINDOWS {
                break;
            }

            window_end = window_start.saturating_sub(1);
        }

        let mut out: Vec<NostrEvent> = by_id.into_values().collect();
        out.sort_by_key(|ev| ev.created_at);
        Ok(out)
    }
}

fn nostr_event_to_model(ev: &nostr_sdk::Event) -> NostrEvent {
    NostrEvent {
        id: ev.id.to_hex(),
        kind: ev.kind.as_u16(),
        pubkey: ev.pubkey.to_hex(),
        created_at: ev.created_at.as_secs(),
        tags: ev.tags.iter().map(|t| t.clone().to_vec()).collect(),
        content: ev.content.clone(),
    }
}
