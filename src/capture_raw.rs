use eframe::epaint::ahash::HashMap;
use escapi::Device;
use image::RgbImage;
use image::{ImageBuffer, Rgb};
use opencv::prelude::Mat;
use opencv::prelude::MatTraitConstManual;
use opencv::prelude::VideoCaptureTrait;
use opencv::videoio::VideoCapture;
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

pub fn capture_with_opencv(cam: &mut VideoCapture) -> anyhow::Result<RgbImage> {
    let mut frame = Mat::default();
    cam.read(&mut frame)?;
    let mut rgb = Mat::default();
    opencv::imgproc::cvt_color(&frame, &mut rgb, opencv::imgproc::COLOR_BGR2RGB, 0)?;
    if rgb.data_bytes()?.len() != WIDTH * HEIGHT * 3 {
        let mut resized = Mat::default();
        // resize
        opencv::imgproc::resize(
            &rgb,
            &mut resized,
            opencv::core::Size_::new(WIDTH as _, HEIGHT as _),
            0.0,
            0.0,
            opencv::imgproc::INTER_LINEAR,
        )?;
        rgb = resized;
    }

    let mut buffer = vec![0; WIDTH * HEIGHT * 3];
    let bytes = rgb.data_bytes()?;
    if bytes.len() != buffer.len() {
        return Err(anyhow::anyhow!(
            "bytes.len() != buffer.len(): {} != {}",
            bytes.len(),
            buffer.len()
        ));
    }
    buffer.copy_from_slice(bytes);
    Ok(ImageBuffer::from_raw(WIDTH as _, HEIGHT as _, buffer).unwrap())
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
