use consumer::Consumer;
use image::ImageBuffer;
use image::Rgb;
use tokio::sync::mpsc;
use tokio::task;

use escapi;

mod consumer;
mod courses;
mod detector;
mod mogi_result;
mod race_result;
mod word;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;

async fn run_producer(tx: mpsc::Sender<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> anyhow::Result<()> {
    println!("producer");
    let camera = escapi::init(0, WIDTH as _, HEIGHT as _, 30).unwrap();
    let (width, height) = (camera.capture_width(), camera.capture_height());
    println!("camera: {}x{}", width, height);

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
                eprintln!("error: {}", e);
                break;
            }
        }
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
        let mut consumer = Consumer;
        let mut mogi_result = mogi_result::MogiResult::new();
        consumer.run(&mut mogi_result, rx).await.unwrap();
    });

    producer.await?;
    consumer.await?;

    Ok(())
}
