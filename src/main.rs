use std::fs::File;
use std::io::Write;

use detector::CourseDetector;
use detector::Detector;
use detector::PositionDetector;
use detector::RaceFinishDetector;
use image::ImageBuffer;
use image::Rgb;
use mogi_result::MogiResult;
use tokio::sync::mpsc;
use tokio::task;

use escapi;

mod courses;
mod detector;
mod mogi_result;
mod race_result;
mod word;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;

async fn run_producer(tx: mpsc::Sender<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> anyhow::Result<()> {
    let mut i = 0;
    println!("producer");
    let camera = escapi::init(0, WIDTH as _, HEIGHT as _, 30).unwrap();
    let (width, height) = (camera.capture_width(), camera.capture_height());
    println!("camera: {}x{}", width, height);

    loop {
        // println!("producer {}", i);
        let pixels = camera.capture().expect("capture failed");

        // convert pixels to RGB.
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
                eprintln!("error: {}", e);
                break;
            }
        }
        i += 1;
    }
    Ok(())
}

async fn run_consumer(mut rx: mpsc::Receiver<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> anyhow::Result<()> {
    let mut i = 0;
    let mut mogi = MogiResult::new();
    let mut last_mogi_state = mogi.clone();
    let mut detector: Box<dyn Detector + Send + Sync> = Box::new(CourseDetector);
    // let mut detector: Box<dyn Detector + Send + Sync> = Box::new(RaceFinishDetector::new());
    // let mut detector: Box<dyn Detector + Send + Sync> = Box::new(PositionDetector);
    while let Some(buffer) = rx.recv().await {
        // println!("consumer {}", i);
        // buffer.save(format!("out/{}.png", i)).unwrap();
        detector = detector.detect(&buffer, &mut mogi).await?;
        if mogi != last_mogi_state {
            println!("mogi: {:?}", mogi);
            last_mogi_state = mogi.clone();
            let mut file = File::create("result.txt")?;
            file.write_all(format!("{mogi}").as_bytes())?;
        }
        i += 1;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel(10);

    let producer = task::spawn(async move {
        run_producer(tx).await.unwrap();
    });

    let consumer = task::spawn(async {
        run_consumer(rx).await.unwrap();
    });

    producer.await?;
    consumer.await?;

    Ok(())
}
