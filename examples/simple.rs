use hue_bt::{client::Client, DeviceSearchFilter};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let client = Client::new(DeviceSearchFilter::Name("Valot"))
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
        time::sleep(Duration::from_millis(100)).await;
    }
}
