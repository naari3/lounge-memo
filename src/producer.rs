use std::{fs::File, io::Write};

use image::RgbImage;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    capture::{Capture, DirectShowCapture, MSMFCapture},
    settings::Settings,
};

#[derive(Debug)]
pub struct Producer;

impl Producer {
    pub async fn run(
        &mut self,
        tx: Sender<RgbImage>,
        mut settings_rx: Receiver<Settings>,
    ) -> anyhow::Result<()> {
        log::info!("producer");
        let mut capture: Option<Box<dyn Capture>> = None;
        loop {
            if let Ok(new_settings) = settings_rx.try_recv() {
                if capture.is_some() {
                    let _ = capture.take();
                }
                capture = if new_settings.directshow() {
                    DirectShowCapture::new(new_settings.device_name())
                        .map(|c| Box::new(c) as Box<dyn Capture>)
                        .map_err(|e| log::error!("DirectShowCapture creation failed: {:?}", e))
                        .ok()
                } else {
                    MSMFCapture::new(new_settings.device_name())
                        .map(|c| Box::new(c) as Box<dyn Capture>)
                        .map_err(|e| log::error!("MSMFCapture creation failed: {:?}", e))
                        .ok()
                };

                if capture.is_none() {
                    continue;
                }

                let mut file = File::create("settings.toml")?;
                let toml = toml::to_string_pretty(&new_settings)?;
                file.write_all(toml.as_bytes())?;
            }
            if let Some(capture) = capture.as_mut() {
                capture.capture(&tx).await?;
                capture.sleep().await;
            }
        }
    }
}
