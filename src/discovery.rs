use futures_core::Stream;
use futures_util::StreamExt;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use tracing::debug;

use wot_td::thing::Thing;

// TODO: move in crate::error later
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("mdns cannot be accessed {0}")]
    Mdns(#[from] mdns_sd::Error),
    #[error("reqwest error {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Missing address")]
    NoAddress,
}

pub type Result<T> = std::result::Result<T, Error>;

const WELL_KNOWN: &str = "/.well-known/wot";

pub struct Discovery {
    mdns: ServiceDaemon,
    service_type: String,
}

async fn get_thing(info: ServiceInfo) -> Result<Thing> {
    let host = info.get_addresses().iter().next().ok_or(Error::NoAddress)?;
    let port = info.get_port();
    let props = info.get_properties();
    let path = props.get("td").map(|s| s.as_str()).unwrap_or(WELL_KNOWN);
    let proto = match props.get("tls") {
        Some(x) if x == "1" => "https",
        _ => "http",
    };

    debug!("Got {proto} {host} {port} {path}");

    let r = reqwest::get(format!("{proto}://{host}:{port}{path}")).await?;

    let t = r.json().await?;

    Ok(t)
}

impl Discovery {
    /// Creates a new Context composed by a series of Thing.
    pub fn new() -> Result<Self> {
        let mdns = ServiceDaemon::new()?;
        let service_type = "_wot._tcp.local.".to_owned();
        Ok(Self { mdns, service_type })
    }

    /// Returns an Stream of discovered things
    pub fn stream(&self) -> Result<impl Stream<Item = Result<Thing>>> {
        let receiver = self.mdns.browse(&self.service_type)?;

        let s = receiver.into_stream().filter_map(|v| async move {
            if let ServiceEvent::ServiceResolved(info) = v {
                let t = get_thing(info).await;
                Some(t)
            } else {
                None
            }
        });

        Ok(s)
    }
}
