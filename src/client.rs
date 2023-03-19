use crate::color;
use crate::error::{Error, Result};
use crate::uuid::Uuid;
use crate::{
    BRIGHTNESS_CHARACTERISTIC, COLOR_CHARACTERISTIC, NAME_CHARACTERISTIC,
    TEMPERATURE_CHARACTERISTIC,
};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::time::Duration;
use tokio::time;

pub enum DeviceSearchFilter<'a> {
    Name(&'a str),
    MAC(&'a str),
}

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
