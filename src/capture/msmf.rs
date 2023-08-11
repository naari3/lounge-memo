use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use escapi::Device;
use image::RgbImage;
use once_cell::sync::Lazy;
use tokio::sync::mpsc::Sender;

use crate::{
    capture_raw::{capture_with_escapi, get_msmf_device_name_map},
    size::{HEIGHT, WIDTH},
};

use super::Capture;

type DeviceCache = HashMap<String, Arc<Mutex<Device>>>;

static DEVICE_CACHE: Lazy<Arc<Mutex<DeviceCache>>> = Lazy::new(|| {
    let map = HashMap::new();
    Arc::new(Mutex::new(map))
});

pub fn open_msmf_device(device_name: &str) -> anyhow::Result<Arc<Mutex<Device>>> {
    if let Some(device) = DEVICE_CACHE.lock().unwrap().get(device_name) {
        return Ok(device.clone());
    }

    let device_name_map = get_msmf_device_name_map()?;
    let device_index = device_name_map
        .into_iter()
        .find(|(_, v)| v == device_name)
        .map(|(k, _)| k)
        .ok_or(anyhow::anyhow!("device_name not found"))?;
    let device = escapi::init(device_index, WIDTH as _, HEIGHT as _, 30)?;
    let (width, height) = (device.capture_width(), device.capture_height());
    log::info!("camera: {} {}x{}", device_name, width, height);
    let device = Arc::new(Mutex::new(device));
    DEVICE_CACHE
        .lock()
        .unwrap()
        .insert(device_name.to_owned(), device.clone());
    Ok(device)
}

pub struct MSMFCapture {
    device: Arc<Mutex<escapi::Device>>,
    last: std::time::Instant,
}

#[async_trait]
impl Capture for MSMFCapture {
    fn new(device_name: &str) -> anyhow::Result<Self> {
        log::info!("MSMFCapture::new({})", device_name);
        let device = open_msmf_device(device_name)?;

        Ok(Self {
            device,
            last: std::time::Instant::now(),
        })
    }

    async fn capture(&mut self, tx: &Sender<RgbImage>) -> anyhow::Result<()> {
        let img = capture_with_escapi(&self.device.lock().unwrap())?;
        tx.send(img).await?;
        self.last = std::time::Instant::now();
        Ok(())
    }

    fn get_last(&self) -> std::time::Instant {
        self.last
    }
}
