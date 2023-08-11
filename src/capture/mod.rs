use async_trait::async_trait;
use image::RgbImage;
use tokio::sync::mpsc::Sender;

mod directshow;
mod msmf;

pub use directshow::DirectShowCapture;
pub use msmf::MSMFCapture;

pub use directshow::open_directshow_device;
pub use msmf::open_msmf_device;

#[async_trait]
pub trait Capture: Send + Sync {
    fn new(device_name: &str) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn capture(&mut self, tx: &Sender<RgbImage>) -> anyhow::Result<()>;

    fn get_last(&self) -> std::time::Instant;

    // 30fpsで駆動
    async fn sleep(&self) {
        let now = std::time::Instant::now();
        let duration = std::time::Duration::from_secs_f64(1.0 / 30.0);
        if now < self.get_last() + duration {
            tokio::time::sleep(duration - (now - self.get_last())).await;
        }
    }
}
