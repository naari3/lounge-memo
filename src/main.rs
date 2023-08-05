use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use clap::Parser;
use consumer::Consumer;
use gui::App;
use image::ImageBuffer;
use image::Rgb;
use log::LevelFilter;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::task;

use crate::capture::loop_capture_with_escapi;
use crate::capture::loop_capture_with_opencv;

mod capture;
mod consumer;
mod courses;
mod detector;
mod gui;
mod mogi_result;
mod race_result;
mod size;
mod word;

#[derive(Debug, Parser, Clone)]
#[clap(name = "lounge-memo")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set a index of camera device
    #[arg(short, long, default_value = "0")]
    index: usize,
    /// Use DirectShow instead of MSMF, default is MSMF.
    /// This is useful when the default does not work well.
    #[arg(short, long, default_value = "false")]
    directshow: bool,
    /// Set a log level
    #[arg(long, default_value = "INFO")]
    log_level: String,
}

fn init_logger(log_level: &str) {
    let log_level = LevelFilter::from_str(log_level).unwrap_or(LevelFilter::Info);
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
        .level_for("lounge_memo", log_level)
        .chain(std::io::stdout());
    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}

async fn run_producer(
    args: &Args,
    tx: mpsc::Sender<ImageBuffer<Rgb<u8>, Vec<u8>>>,
) -> anyhow::Result<()> {
    let camera_index = args.index;
    log::info!("producer");
    if args.directshow {
        loop_capture_with_opencv(camera_index, tx).await?;
    } else {
        loop_capture_with_escapi(camera_index, tx).await?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger(&args.log_level);
    let rt = Runtime::new().expect("Unable to create Runtime");

    let (from_gui_tx, from_gui_rx) = mpsc::channel(10);
    let (to_gui_tx, to_gui_rx) = mpsc::channel(10);

    let (tx, rx) = mpsc::channel(10);

    std::thread::spawn(move || {
        rt.block_on(async {
            let producer = task::spawn(async move {
                run_producer(&args, tx).await.unwrap();
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
    options.initial_window_size = Some(eframe::egui::Vec2::new(400.0, 720.0));
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
