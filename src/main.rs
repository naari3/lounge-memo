use std::sync::Arc;
use std::sync::Mutex;

use consumer::Consumer;
use gui::App;
use image::ImageBuffer;
use image::Rgb;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::task;

use escapi;

mod consumer;
mod courses;
mod detector;
mod gui;
mod mogi_result;
mod race_result;
mod word;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;

fn init_logger() {
    let base_config = fern::Dispatch::new();
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // .level(log::LevelFilter::Trace)
        .level(log::LevelFilter::Warn)
        .level_for("lounge_memo", log::LevelFilter::Trace)
        .chain(fern::log_file("output.log").unwrap());
    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // .level(log::LevelFilter::Trace)
        .level(log::LevelFilter::Warn)
        .level_for("lounge_memo", log::LevelFilter::Info)
        .chain(std::io::stdout());
    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}

async fn run_producer(tx: mpsc::Sender<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> anyhow::Result<()> {
    // 引数から数字を取得する なければ0
    let camera_index = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    log::info!("producer");
    let camera = escapi::init(camera_index, WIDTH as _, HEIGHT as _, 30).unwrap();
    let (width, height) = (camera.capture_width(), camera.capture_height());
    log::info!("camera: {} {}x{}", camera.name(), width, height);

    loop {
        let pixels = camera.capture().expect("capture failed");

        let mut buffer = vec![0; width as usize * height as usize * 3];
        for i in 0..pixels.len() / 4 {
            buffer[i * 3] = pixels[i * 4 + 2];
            buffer[i * 3 + 1] = pixels[i * 4 + 1];
            buffer[i * 3 + 2] = pixels[i * 4];
        }

        let img = ImageBuffer::from_raw(width as _, height as _, buffer).unwrap();
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

fn main() -> anyhow::Result<()> {
    init_logger();
    let rt = Runtime::new().expect("Unable to create Runtime");

    let (from_gui_tx, from_gui_rx) = mpsc::channel(10);
    let (to_gui_tx, to_gui_rx) = mpsc::channel(10);

    let (tx, rx) = mpsc::channel(10);

    std::thread::spawn(move || {
        rt.block_on(async {
            let producer = task::spawn(async move {
                run_producer(tx).await.unwrap();
            });

            let consumer = task::spawn(async {
                let mut consumer = Consumer;
                let mut mogi_result = mogi_result::MogiResult::new();
                consumer
                    .run(&mut mogi_result, rx, to_gui_tx, from_gui_rx)
                    .await
                    .unwrap();
            });

            producer.await.unwrap();
            consumer.await.unwrap();
        });
    });

    let mut options = eframe::NativeOptions::default();
    // options.always_on_top = true;
    options.initial_window_size = Some(eframe::egui::Vec2::new(400.0, 600.0));
    eframe::run_native(
        "lounge-memo",
        options,
        Box::new(|ctx| {
            Box::new(App::new(
                ctx,
                Arc::new(Mutex::new(from_gui_tx)),
                Arc::new(Mutex::new(to_gui_rx)),
            ))
        }),
    )
    .unwrap();

    Ok(())
}
