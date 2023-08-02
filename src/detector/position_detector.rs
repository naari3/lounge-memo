use async_trait::async_trait;

use super::Detector;
use crate::detector::{CaptureTotalScoresDetector, CourseDetector};
use crate::race_result::Position;
use crate::HEIGHT;
use crate::{mogi_result::MogiResult, WIDTH};
use image::Rgb;
use image::{ImageBuffer, Pixel};

pub struct PositionDetector {
    positions_vec: Vec<Position>,
}

const LINE_HEIGHT: f64 = (78.0 / 1080.0) * HEIGHT as f64;
const LINES: usize = 12;
const LINES_SAMPLE_OFFSET_Y: f64 = 81.0 / 1080.0 * HEIGHT as f64;
const LINES_SAMPLE_OFFSET_X: f64 = WIDTH as f64 - (220.0 / 1920.0 * WIDTH as f64);

impl PositionDetector {
    pub fn new() -> PositionDetector {
        log::info!("PositionDetector");
        PositionDetector {
            positions_vec: Vec::new(),
        }
    }
}

#[async_trait]
impl Detector for PositionDetector {
    async fn detect(
        mut self: Box<Self>,
        buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        mogi_result: &mut MogiResult,
    ) -> anyhow::Result<Box<dyn Detector + Send + Sync>> {
        if self.detect_error(buffer, mogi_result).await? {
            return Ok(Box::new(CourseDetector::new()));
        }

        // sample pixel of each line, and check if it's yellow or not
        let sample_pixels = (0..LINES)
            .enumerate()
            .map(|(index, i)| {
                let x = LINES_SAMPLE_OFFSET_X;
                let y_offset = LINES_SAMPLE_OFFSET_Y + LINE_HEIGHT * i as f64;
                let mut pixels = Vec::new();
                log::trace!("index: {}", index);
                for y in (y_offset as u32)..(y_offset as u32) + 5 {
                    let pixel = buffer.get_pixel(x as u32, y as u32).clone();
                    log::trace!("x: {x}, y: {y}, color: {:?}", pixel.channels());
                    pixels.push(pixel);
                }
                pixels
            })
            .collect::<Vec<Vec<Rgb<u8>>>>();

        // check which index pixels is yellow zone.
        let yellow_line_index = sample_pixels
            .into_iter()
            .enumerate()
            .find(|(_, p)| is_yellow_zone(p))
            .map(|(i, _)| i);

        if let Some(yellow_line_index) = yellow_line_index {
            let position = Position::from_index(yellow_line_index);
            mogi_result.set_current_position(position);
            self.positions_vec.push(position);
            // すべて同じPositionだったら
            if self.positions_vec.len() >= 4
                && self
                    .positions_vec
                    .iter()
                    .all(|p| *p == self.positions_vec[0])
            {
                log::info!("position: {position}");
                mogi_result.save_result_image(buffer, "race")?;
                return Ok(Box::new(CaptureTotalScoresDetector::new()));
            }
            // ひとつでも違うPositionがあったら
            if self
                .positions_vec
                .iter()
                .any(|p| *p != self.positions_vec[0])
            {
                self.positions_vec.clear();
            }
        }

        Ok(self)
    }
}

fn is_yellow_zone(pixels: &[Rgb<u8>]) -> bool {
    pixels.iter().all(|p| is_yellow(p))
}

fn is_yellow(pixel: &Rgb<u8>) -> bool {
    let channels = pixel.channels();
    let r = channels[0];
    let g = channels[1];
    let b = channels[2];
    // 0xD0 = 208
    // 0xC8 = 200
    // 0x80 = 128
    r > 0xD0 && g > 0xC8 && b < 0x80
}
