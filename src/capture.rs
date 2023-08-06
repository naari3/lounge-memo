use eframe::epaint::ahash::HashMap;
use escapi::Device;
use image::{ImageBuffer, Rgb};
use opencv::prelude::Mat;
use opencv::prelude::MatTraitConstManual;
use opencv::videoio::VideoCapture;
use opencv::videoio::CAP_PROP_FRAME_HEIGHT;
use opencv::videoio::CAP_PROP_FRAME_WIDTH;
use opencv::{
    prelude::{VideoCaptureTrait, VideoCaptureTraitConst},
    videoio,
};
use tokio::sync::mpsc;
use windows::core::Interface;
use windows::w;
use windows::Win32::Media::DirectShow::ICreateDevEnum;
use windows::Win32::Media::MediaFoundation::CLSID_SystemDeviceEnum;
use windows::Win32::Media::MediaFoundation::CLSID_VideoInputDeviceCategory;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CoInitialize;
use windows::Win32::System::Com::IEnumMoniker;
use windows::Win32::System::Com::IMoniker;
use windows::Win32::System::Com::StructuredStorage::IPropertyBag;
use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;

use crate::size::HEIGHT;
use crate::size::WIDTH;

pub async fn loop_capture_with_opencv(
    camera_index: usize,
    tx: mpsc::Sender<ImageBuffer<Rgb<u8>, Vec<u8>>>,
) -> anyhow::Result<()> {
    let friendly_names = get_friendly_names()?;
    let mut cam = videoio::VideoCapture::new(camera_index as _, videoio::CAP_ANY)?;
    cam.set(CAP_PROP_FRAME_WIDTH, WIDTH as f64)?;
    cam.set(CAP_PROP_FRAME_HEIGHT, HEIGHT as f64)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    log::info!(
        "camera: {} {}x{}",
        friendly_names[camera_index],
        cam.get(CAP_PROP_FRAME_WIDTH)?,
        cam.get(CAP_PROP_FRAME_HEIGHT)?,
    );

    loop {
        let img = capture_with_opencv(&mut cam)?;
        match tx.send(img).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

pub fn capture_with_opencv(
    cam: &mut VideoCapture,
) -> anyhow::Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let mut frame = Mat::default();
    cam.read(&mut frame)?;

    let mut resized = Mat::default();
    opencv::imgproc::resize(
        &frame,
        &mut resized,
        opencv::core::Size {
            width: WIDTH as _,
            height: HEIGHT as _,
        },
        0.0,
        0.0,
        opencv::imgproc::INTER_LINEAR,
    )?;
    // convert to RGB
    let mut rgb = Mat::default();
    opencv::imgproc::cvt_color(&resized, &mut rgb, opencv::imgproc::COLOR_BGR2RGB, 0)?;
    // convert to ImageBuffer
    let mut buffer = vec![0; WIDTH * HEIGHT * 3];
    let bytes = rgb.data_bytes()?;
    buffer.copy_from_slice(bytes);
    Ok(ImageBuffer::from_raw(WIDTH as _, HEIGHT as _, buffer).unwrap())
}

pub async fn loop_capture_with_escapi(
    camera_index: usize,
    tx: mpsc::Sender<ImageBuffer<Rgb<u8>, Vec<u8>>>,
) -> anyhow::Result<()> {
    let cam = escapi::init(camera_index, WIDTH as _, HEIGHT as _, 30).unwrap();
    let (width, height) = (cam.capture_width(), cam.capture_height());
    log::info!("camera: {} {}x{}", cam.name(), width, height);

    loop {
        let img = capture_with_escapi(&cam)?;
        match tx.send(img).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

pub fn capture_with_escapi(cam: &Device) -> anyhow::Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let (width, height) = (cam.capture_width(), cam.capture_height());
    let pixels = cam.capture().expect("capture failed");

    let mut buffer = vec![0; width as usize * height as usize * 3];
    for i in 0..pixels.len() / 4 {
        buffer[i * 3] = pixels[i * 4 + 2];
        buffer[i * 3 + 1] = pixels[i * 4 + 1];
        buffer[i * 3 + 2] = pixels[i * 4];
    }

    Ok(ImageBuffer::from_raw(width as _, height as _, buffer).unwrap())
}

fn get_friendly_names() -> anyhow::Result<Vec<String>> {
    let mut friendly_names = Vec::new();

    unsafe {
        CoInitialize(None)?;

        let create_dev_enum: ICreateDevEnum =
            CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_INPROC_SERVER)?;

        let mut em: Option<IEnumMoniker> = None;
        create_dev_enum.CreateClassEnumerator(&CLSID_VideoInputDeviceCategory, &mut em, 0)?;

        loop {
            let mut moniker: Vec<Option<IMoniker>> = vec![None];
            let hr = em
                .as_ref()
                .unwrap()
                .Next(&mut moniker, Some(std::ptr::null_mut()));
            if hr.is_err() {
                break;
            }

            let moniker = moniker.get(0).unwrap();
            let moniker = match moniker {
                Some(moniker) => moniker,
                None => break,
            };
            let mut bag: std::mem::MaybeUninit<IPropertyBag> = std::mem::MaybeUninit::zeroed();
            moniker.BindToStorage(
                None,
                None,
                &IPropertyBag::IID,
                bag.as_mut_ptr() as *mut *mut _,
            )?;

            let bag = bag.assume_init();
            let mut variant = windows::Win32::System::Com::VARIANT::default();
            let key = w!("FriendlyName");

            bag.Read(key, &mut variant, None)?;
            let friendly_name = variant.Anonymous.Anonymous.Anonymous.bstrVal.to_string();

            friendly_names.push(friendly_name);
        }
    }

    Ok(friendly_names)
}

pub fn get_msmf_device_name_map() -> anyhow::Result<HashMap<usize, String>> {
    let device_num = escapi::num_devices();
    let device_map = (0..device_num)
        .filter_map(|i| {
            escapi::init(i, WIDTH as _, HEIGHT as _, 30)
                .ok()
                .map(|r| (i, r.name()))
        })
        .collect::<HashMap<usize, String>>();

    Ok(device_map)
}

pub fn get_directshow_device_name_map() -> anyhow::Result<HashMap<usize, String>> {
    let mut friendly_names = get_friendly_names()?;
    let device_map = (0..friendly_names.len())
        .map(|i| (i, friendly_names.remove(0)))
        .collect::<HashMap<usize, String>>();

    Ok(device_map)
}
