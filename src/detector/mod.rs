use async_trait::async_trait;
use image::ImageBuffer;
use image::Rgb;

use crate::mogi_result::MogiResult;
use crate::word::normalize_japanese_characters;
use crate::word::words_from_image_buffer;

mod course_detector;
mod position_detector;
mod race_finish_detector;

pub use course_detector::CourseDetector;
pub use position_detector::PositionDetector;
pub use race_finish_detector::RaceFinishDetector;

#[async_trait]
pub trait Detector {
    async fn detect(
        self: Box<Self>,
        buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        mogi_result: &mut MogiResult,
    ) -> anyhow::Result<Box<dyn Detector + Send + Sync>>;

    async fn detect_error(
        &self,
        buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        mogi_result: &mut MogiResult,
    ) -> anyhow::Result<bool> {
        let words =
            words_from_image_buffer(buffer, buffer.width() as _, buffer.height() as _).await?;
        let normalized_words = words
            .into_iter()
            .filter(|w| w.text.len() >= 2)
            .map(|w| normalize_japanese_characters(w.text.replace(" ", "")))
            .collect::<Vec<String>>();

        let mut error_count = 0;
        for word in &normalized_words {
            for error_word in &["エラー", "通信", "はっせい", "しました"] {
                if word.contains(&normalize_japanese_characters(error_word.to_string())) {
                    error_count += 1;
                }
                if error_count == 4 {
                    log::warn!("エラーが発生しました");
                    mogi_result.reset_current_course();
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
