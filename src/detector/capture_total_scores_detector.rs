use std::time::Instant;

use async_trait::async_trait;

use super::{CourseDetector, Detector};

pub struct CaptureTotalScoresDetector {
    position_checked_at: Instant,
}

impl CaptureTotalScoresDetector {
    pub fn new(position_checked_at: Instant) -> CaptureTotalScoresDetector {
        log::info!("CaptureTotalScoresDetector");
        CaptureTotalScoresDetector {
            position_checked_at,
        }
    }
}

#[async_trait]
impl Detector for CaptureTotalScoresDetector {
    async fn detect(
        mut self: Box<Self>,
        buffer: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
        mogi_result: &mut crate::mogi_result::MogiResult,
    ) -> anyhow::Result<Box<dyn Detector + Send + Sync>> {
        if self.position_checked_at.elapsed().as_secs_f64() < 4.0 {
            return Ok(self);
        }

        log::info!("capture total scores");
        mogi_result.save_result_image(buffer, "total")?;
        return Ok(Box::new(CourseDetector::new()));
    }
}
