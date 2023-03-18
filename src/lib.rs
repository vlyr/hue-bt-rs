use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType, CharPropFlags};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

pub mod color;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    central.start_scan(ScanFilter::default()).await?;
    time::sleep(Duration::from_secs(5)).await;

    let light = find_light(&central).await.unwrap();

    light.connect().await?;

    light.discover_services().await?;

    let chars = light.characteristics();
    // party
    let colors = &["#FF0000", "#00FFFF", "#FF00FF", "#0000FF"]; 
    let mut idx = 0;
    for x in chars.iter().filter(|x| x.uuid == Uuid::parse_str("932c32bd-0005-47a2-835a-a8d455b859dd").unwrap()) {
        println!("{}", x.uuid);
        for i in 0..1000 {
            light.write(x, &color::color(colors[idx]).unwrap(), WriteType::WithoutResponse).await?;
            if idx == 3 {
                idx = 0;
            } else {
                idx += 1;
            }
            time::sleep(Duration::from_millis(50)).await;
        }
    }

    Ok(())
}

async fn find_light(central: &Adapter) -> Option<Peripheral> {
    println!("{:#?}", central.peripherals().await.unwrap().iter().map(|x| async { x.properties().await.unwrap().unwrap() }));
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Lights"))
        {
            return Some(p);
        }
    }
    None
}
