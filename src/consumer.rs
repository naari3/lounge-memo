use std::{fs::File, io::Write};

use fps_counter::FPSCounter;
use image::{ImageBuffer, Rgb};
use tokio::sync::mpsc;

use crate::{
    detector::{CourseDetector, Detector},
    mogi_result::MogiResult,
};

#[derive(Debug)]
pub struct Consumer;

impl Consumer {
    pub async fn run(
        &mut self,
        mogi_result: &mut MogiResult,
        mut rx: mpsc::Receiver<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    ) -> anyhow::Result<()> {
        let mut a = FPSCounter::default();
        let mut i = 0;
        let mut last_mogi_state = mogi_result.clone();
        let mut detector: Box<dyn Detector + Send + Sync> = Box::new(CourseDetector::new());
        // let mut detector: Box<dyn Detector + Send + Sync> = Box::new(RaceFinishDetector::new());
        // let mut detector: Box<dyn Detector + Send + Sync> = Box::new(PositionDetector::new());
        while let Some(buffer) = rx.recv().await {
            a.tick();
            if i % 10 == 0 {
                println!("{:?}", a.tick());
            }
            detector = detector.detect(&buffer, mogi_result).await?;
            if mogi_result != &last_mogi_state {
                println!("mogi: {:?}", mogi_result);
                last_mogi_state = mogi_result.clone();
                let mut file = File::create("result.txt")?;
                file.write_all(format!("{mogi_result}").as_bytes())?;
            }
            i += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate ffmpeg_next as ffmpeg;

    use std::path::Path;

    use ffmpeg::format::{input, Pixel};
    use ffmpeg::media::Type;
    use ffmpeg::software::scaling::{context::Context, flag::Flags};
    use ffmpeg::util::frame::video::Video;
    use image::{ImageBuffer, Rgb};

    use crate::mogi_result::MogiResult;

    use super::Consumer;

    fn decode_video<P: AsRef<Path>>(
        path: &P,
    ) -> anyhow::Result<Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>> {
        let mut frames = Vec::new();
        let mut ictx = input(path)?;
        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut frame_index = 0;

        fn receive_and_process_decoded_frames(
            decoder: &mut ffmpeg::decoder::Video,
            scaler: &mut Context,
            frames_vec: &mut Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>,
            frame_index: &mut i32,
        ) -> Result<(), ffmpeg::Error> {
            let mut decoded: Video = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                let rgb_buffer = rgb_frame.data(0);
                if let Some(image_buffer) = ImageBuffer::from_raw(
                    rgb_frame.width(),
                    rgb_frame.height(),
                    rgb_buffer.to_vec(),
                ) {
                    println!("frame_index: {}", frame_index);
                    frames_vec.push(image_buffer);
                }
                *frame_index += 1;
            }
            Ok(())
        }

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(
                    &mut decoder,
                    &mut scaler,
                    &mut frames,
                    &mut frame_index,
                )?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(
            &mut decoder,
            &mut scaler,
            &mut frames,
            &mut frame_index,
        )?;
        Ok(frames)
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let mut mogi_result = MogiResult::new();
        ffmpeg::init().unwrap();

        let mut consumer = Consumer;
        let (tx, rx) = tokio::sync::mpsc::channel(10);
        let frames = tokio::task::spawn_blocking(move || {
            decode_video(&"./test_assets/input.mp4".to_string()).unwrap()
        })
        .await?;

        let producer = tokio::task::spawn(async move {
            for frame in frames {
                tx.send(frame).await.unwrap();
            }
        });
        let consumer = tokio::task::spawn(async move {
            consumer.run(&mut mogi_result, rx).await.unwrap();
        });

        producer.await?;
        consumer.await?;

        // mp4 to ImageBuffers
        Ok(())
    }
}
