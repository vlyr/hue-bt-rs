use crate::error::{Error, Result};
use crate::uuid::Uuid;
use crate::{color, DeviceSearchFilter, LightState};
use crate::{
    BRIGHTNESS_CHARACTERISTIC, COLOR_CHARACTERISTIC, NAME_CHARACTERISTIC, STATE_CHARACTERISTIC,
    TEMPERATURE_CHARACTERISTIC,
};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::time::Duration;
use tokio::time;

/// Basic client abstraction that has a bluetooth peripheral as the inner layer.
///```
/// use hue_bt::{client::Client, DeviceSearchFilter};
/// use std::time::Duration;
/// use tokio::time;

/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(DeviceSearchFilter::Name("Lights"))
///         .await
///         .unwrap();
///
///     client.set_color("#FF0000").await.unwrap();
/// }
///```
pub struct Client {
    inner: Peripheral,
}

impl Client {
    pub async fn new<'a>(filter: DeviceSearchFilter<'a>) -> Result<Self> {
        let manager = Manager::new().await?;

        let adapters = manager.adapters().await?;

        let central = adapters
            .into_iter()
            .nth(0)
            .expect("Could not find bluetooth adapter");

        central.start_scan(ScanFilter::default()).await?;

        time::sleep(Duration::from_millis(1000)).await;

        let light = find_light(&central, filter)
            .await
            .ok_or(Error::LightNotFound)?;

        light.connect().await?;
        light.discover_services().await?;

        Ok(Self { inner: light })
    }

    pub async fn set_brightness(&self, value: u8) -> Result<()> {
        let chars = self.inner.characteristics();

        if let Some(c) = chars
            .iter()
            .find(|x| x.uuid == Uuid::parse_str(BRIGHTNESS_CHARACTERISTIC).unwrap())
        {
            self.inner
                .write(c, &[value], WriteType::WithoutResponse)
                .await?;
        }
        Ok(())
    }

    pub async fn set_temperature(&self, value: u8) -> Result<()> {
        let chars = self.inner.characteristics();

        if let Some(c) = chars
            .iter()
            .find(|x| x.uuid == Uuid::parse_str(TEMPERATURE_CHARACTERISTIC).unwrap())
        {
            self.inner
                .write(c, &[value], WriteType::WithoutResponse)
                .await?;
        }
        Ok(())
    }

    pub async fn set_state(&self, value: LightState) -> Result<()> {
        let chars = self.inner.characteristics();

        if let Some(c) = chars
            .iter()
            .find(|x| x.uuid == Uuid::parse_str(STATE_CHARACTERISTIC).unwrap())
        {
            self.inner
                .write(c, &[value as u8], WriteType::WithoutResponse)
                .await?;
        }
        Ok(())
    }

    pub async fn read_state(&self) -> Option<LightState> {
        let chars = self.inner.characteristics();

        match chars
            .iter()
            .find(|x| x.uuid == Uuid::parse_str(STATE_CHARACTERISTIC).unwrap())
        {
            Some(c) => {
                let value = self.inner.read(c).await.unwrap()[0];
                Some(LightState::from(value))
            }
            None => None,
        }
    }

    /// Reads the state of the light and writes the opposite state value.
    pub async fn toggle(&self) -> Result<()> {
        let current_state = self.read_state().await;

        if let Some(curr) = current_state {
            let new_state = match curr {
                LightState::On => LightState::Off,
                LightState::Off => LightState::On,
            };

            self.set_state(new_state).await?;
        }

        Ok(())
    }
    /// Sets the colors of the light. `hex_string` is a 6-digit hex string, with or without a #-prefix.
    pub async fn set_color<T>(&self, hex_string: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let chars = self.inner.characteristics();

        if let Some(c) = chars
            .iter()
            .find(|x| x.uuid == Uuid::parse_str(COLOR_CHARACTERISTIC).unwrap())
        {
            self.inner
                .write(
                    c,
                    &color::color(hex_string.as_ref())?,
                    WriteType::WithoutResponse,
                )
                .await?;
        }
        Ok(())
    }

    pub async fn set_light_name<T>(&self, name: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let chars = self.inner.characteristics();

        if let Some(c) = chars
            .iter()
            .find(|x| x.uuid == Uuid::parse_str(NAME_CHARACTERISTIC).unwrap())
        {
            self.inner
                .write(c, &name.as_ref().as_bytes(), WriteType::WithoutResponse)
                .await?;
        }
        Ok(())
    }
}

async fn find_light<'a>(
    central: &Adapter,
    search_filter: DeviceSearchFilter<'a>,
) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        match search_filter {
            DeviceSearchFilter::Name(ref n) => {
                if p.properties()
                    .await
                    .unwrap()
                    .unwrap()
                    .local_name
                    .iter()
                    .any(|name| name.contains(n))
                {
                    return Some(p);
                }
            }
            DeviceSearchFilter::MAC(ref m) => {
                if p.properties()
                    .await
                    .unwrap()
                    .unwrap()
                    .address
                    .to_string()
                    .contains(m)
                {
                    return Some(p);
                }
            }
        }
    }
    None
}
