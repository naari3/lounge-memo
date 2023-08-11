use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use image::RgbImage;
use once_cell::sync::Lazy;
use opencv::prelude::VideoCaptureTraitConst;
use opencv::videoio::{self, VideoCaptureTrait, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use tokio::sync::mpsc::Sender;

use crate::capture_raw::capture_with_opencv;
use crate::{
    capture_raw::get_directshow_device_name_map,
    size::{HEIGHT, WIDTH},
};

use super::Capture;

type DeviceCache = HashMap<String, Arc<Mutex<videoio::VideoCapture>>>;

static DEVICE_CACHE: Lazy<Arc<Mutex<DeviceCache>>> = Lazy::new(|| {
    let map = HashMap::new();
    Arc::new(Mutex::new(map))
});

pub fn open_directshow_device(
    device_name: &str,
) -> anyhow::Result<Arc<Mutex<videoio::VideoCapture>>> {
    if let Some(device) = DEVICE_CACHE.lock().unwrap().get(device_name) {
        return Ok(device.clone());
    }

    let device_name_map = get_directshow_device_name_map()?;
    let device_index = device_name_map
        .into_iter()
        .find(|(_, v)| v == device_name)
        .map(|(k, _)| k)
        .ok_or(anyhow::anyhow!("device_name not found"))?;
    let mut device = videoio::VideoCapture::new(device_index as _, videoio::CAP_DSHOW)?;
    device.set(CAP_PROP_FRAME_WIDTH, WIDTH as f64)?;
    device.set(CAP_PROP_FRAME_HEIGHT, HEIGHT as f64)?;
    let opened = videoio::VideoCapture::is_opened(&device)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    log::info!(
        "camera: {} {}x{}",
        device_name,
        device.get(CAP_PROP_FRAME_WIDTH)?,
        device.get(CAP_PROP_FRAME_HEIGHT)?,
    );
    let device = Arc::new(Mutex::new(device));
    DEVICE_CACHE
        .lock()
        .unwrap()
        .insert(device_name.to_owned(), device.clone());
    Ok(device)
}

#[derive(Debug)]
pub struct DirectShowCapture {
    device: Arc<Mutex<videoio::VideoCapture>>,
    last: std::time::Instant,
}

#[async_trait]
impl Capture for DirectShowCapture {
    fn new(device_name: &str) -> anyhow::Result<Self> {
        log::info!("DirectShowCapture::new({})", device_name);
        let device = open_directshow_device(device_name)?;

        Ok(Self {
            device,
            last: std::time::Instant::now(),
        })
    }

    async fn capture(&mut self, tx: &Sender<RgbImage>) -> anyhow::Result<()> {
        let img = match capture_with_opencv(&mut self.device.lock().unwrap()) {
            Ok(img) => img,
            Err(e) => {
                log::error!("capture_with_opencv failed: {}", e);
                return Ok(());
            }
        };
        tx.send(img).await?;
        self.last = std::time::Instant::now();
        Ok(())
    }

    fn get_last(&self) -> std::time::Instant {
        self.last
    }
}
