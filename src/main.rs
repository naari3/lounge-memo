use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use consumer::Consumer;
use gui::App;
use log::LevelFilter;
use mogi_result::MogiResult;
use settings::Settings;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::task;

use crate::producer::Producer;

mod capture;
mod capture_raw;
mod consumer;
mod courses;
mod detector;
mod gui;
mod mogi_result;
mod producer;
mod race_result;
mod settings;
mod size;
mod word;

fn init_logger(log_level: &str, write_log_to_file: bool) {
    let log_level = LevelFilter::from_str(log_level).unwrap_or(LevelFilter::Info);
    let mut base_config = fern::Dispatch::new();
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
    base_config = base_config.chain(stdout_config);

    if write_log_to_file {
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

        base_config = base_config.chain(file_config);
    }

    base_config.apply().unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings: Settings = match read_to_string("settings.toml") {
        Ok(s) => toml::from_str(&s).unwrap(),
        Err(_) => Settings::new("OBS-Camera".to_string(), true, "INFO".to_string(), false),
    };
    init_logger(settings.log_level(), settings.write_log_to_file());

    let mut result: MogiResult = match read_to_string("result.json") {
        Ok(s) => serde_json::from_str(&s).unwrap(),
        Err(_) => MogiResult::new(),
    };

    let rt = Runtime::new().expect("Unable to create Runtime");

    let (settings_tx, settings_rx) = mpsc::channel(10);

    settings_tx.send(settings.clone()).await.unwrap();

    let (from_gui_tx, from_gui_rx) = mpsc::channel(10);
    let (to_gui_tx, to_gui_rx) = mpsc::channel(10);

    let (tx, rx) = mpsc::channel(10);

    std::thread::spawn(move || {
        rt.block_on(async {
            let producer = task::spawn(async move {
                let mut producer = Producer;
                producer.run(tx, settings_rx).await.unwrap();
            });

            let consumer = task::spawn(async move {
                let mut consumer = Consumer;
                consumer
                    .run(&mut result, rx, to_gui_tx, from_gui_rx)
                    .await
                    .unwrap();
            });

            match producer.await {
                Ok(_) => {
                    log::info!("producer finished");
                    consumer.abort();
                }
                Err(err) => {
                    log::error!("producer error: {}", err);
                    consumer.abort();
                }
            };
            match consumer.await {
                Ok(_) => {
                    log::info!("consumer finished");
                }
                Err(err) => {
                    log::error!("consumer error: {}", err);
                }
            }

            log::info!("finish");
            // exit process
            std::process::exit(0);
        });
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::Vec2::new(400.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "lounge-memo",
        options,
        Box::new(|ctx| {
            Box::new(App::new(
                ctx,
                Arc::new(Mutex::new(from_gui_tx)),
                Arc::new(Mutex::new(to_gui_rx)),
                Arc::new(Mutex::new(settings_tx)),
                settings,
            ))
        }),
    )
    .unwrap();

    Ok(())
}
