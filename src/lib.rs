pub mod client;
pub mod color;
pub use uuid;
pub mod error;

pub const COLOR_CHARACTERISTIC: &str = "932c32bd-0005-47a2-835a-a8d455b859dd";
pub const TEMPERATURE_CHARACTERISTIC: &str = "932c32bd-0004-47a2-835a-a8d455b859dd";
pub const BRIGHTNESS_CHARACTERISTIC: &str = "932c32bd-0003-47a2-835a-a8d455b859dd";
pub const NAME_CHARACTERISTIC: &str = "97fe6561-0003-4f62-86e9-b71ee2da3d22";
pub const STATE_CHARACTERISTIC: &str = "932c32bd-0002-47a2-835a-a8d455b859dd";

/// Used for searching for a bluetooth device with the given MAC address (DeviceSearchFilter::MAC)
/// or it's user-defined name (DeviceSearchFilter::Name).
pub enum DeviceSearchFilter<'a> {
    Name(&'a str),
    MAC(&'a str),
}

/// Abstraction for Off = 0 and On = 1.
#[repr(u8)]
pub enum LightState {
    Off = 0x00,
    On = 0x01,
}

impl From<u8> for LightState {
    fn from(data: u8) -> Self {
        match data {
            0 => LightState::Off,
            1 => LightState::On,
            _ => panic!("Invalid light state provided."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use std::env;
    use std::time::Duration;
    use tokio::time;

    #[tokio::test]
    async fn party() {
        let light = env::var("LIGHT_NAME").unwrap();
        let client = Client::new(DeviceSearchFilter::Name(&light)).await.unwrap();

        let colors = &["#FF0000", "#00FFFF", "#FF00FF", "#0000FF"];
        let mut idx = 0;

        for _ in 0..1000 {
            client.set_color(colors[idx]).await.unwrap();

            if idx == 3 {
                idx = 0;
            } else {
                idx += 1;
            }
            time::sleep(Duration::from_millis(300)).await;
        }
    }

    #[tokio::test]
    async fn toggle_state() {
        let light = env::var("LIGHT_NAME").unwrap();
        let client = Client::new(DeviceSearchFilter::Name(&light)).await.unwrap();

        for _ in 0..10 {
            client.toggle().await.unwrap();
            time::sleep(Duration::from_millis(300)).await;
        }
    }
    #[tokio::test]
    async fn set_name() {
        let light = env::var("LIGHT_NAME").unwrap();
        let client = Client::new(DeviceSearchFilter::Name(&light)).await.unwrap();

        client.set_light_name("Hue BT Rust testing").await.unwrap();
    }

    #[tokio::test]
    async fn set_temperature() {
        let light = env::var("LIGHT_NAME").unwrap();
        let client = Client::new(DeviceSearchFilter::Name(&light)).await.unwrap();

        client.set_temperature(40, 1).await.unwrap();
    }

    #[tokio::test]
    async fn get_temperature() {
        let light = env::var("LIGHT_NAME").unwrap();
        let client = Client::new(DeviceSearchFilter::Name(&light)).await.unwrap();

        client.get_temperature().await.unwrap();
    }
    #[tokio::test]
    async fn get_all_values() {
        let light = env::var("LIGHT_NAME").unwrap();
        let client = Client::new(DeviceSearchFilter::Name(&light)).await.unwrap();

        client.get_all_values().await.unwrap();
    }
}
