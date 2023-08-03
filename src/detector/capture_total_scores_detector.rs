use async_trait::async_trait;

use super::{CourseDetector, Detector};

pub struct CaptureTotalScoresDetector {
    delay_timer: usize,
}

impl CaptureTotalScoresDetector {
    pub fn new() -> CaptureTotalScoresDetector {
        log::debug!("CaptureTotalScoresDetector");
        CaptureTotalScoresDetector { delay_timer: 120 }
    }
}

#[async_trait]
impl Detector for CaptureTotalScoresDetector {
    async fn detect(
        mut self: Box<Self>,
        buffer: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
        mogi_result: &mut crate::mogi_result::MogiResult,
    ) -> anyhow::Result<Box<dyn Detector + Send + Sync>> {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            return Ok(self);
        } else {
            mogi_result.save_result_image(buffer, "total")?;
            return Ok(Box::new(CourseDetector::new()));
        }
    }
}
