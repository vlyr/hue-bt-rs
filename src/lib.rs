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
    Off = 0,
    On = 1,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use std::time::Duration;
    use tokio::time;

    #[tokio::test]
    async fn party() {
        let client = Client::new(DeviceSearchFilter::Name("Lights"))
            .await
            .unwrap();

        let colors = &["#FF0000", "#00FFFF", "#FF00FF", "#0000FF"];
        let mut idx = 0;

        for _ in 0..1000 {
            client.set_color(colors[idx]).await.unwrap();

            if idx == 3 {
                idx = 0;
            } else {
                idx += 1;
            }
            time::sleep(Duration::from_millis(2000)).await;
        }
    }
}
