use async_trait::async_trait;
use image::ImageBuffer;
use image::Rgb;
use windows::{
    core::Interface,
    Graphics::Imaging::{BitmapBufferAccessMode, BitmapPixelFormat, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Win32::System::WinRT::IMemoryBufferByteAccess,
};

use crate::courses::normalize_japanese_characters;
use crate::{mogi_result::MogiResult, word::Word};

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
            .map(|w| normalize_japanese_characters(w.text.replace(" ", "")))
            .collect::<Vec<String>>();

        if normalized_words.contains(&normalize_japanese_characters("エラー".to_string()))
            && normalized_words.contains(&normalize_japanese_characters("通信".to_string()))
            && normalized_words.contains(&normalize_japanese_characters("はっせい".to_string()))
            && normalized_words.contains(&normalize_japanese_characters("しました".to_string()))
        {
            println!("通信エラーが発生しました");
            mogi_result.reset_current_course();
            return Ok(true);
        }
        Ok(false)
    }
}

fn make_bmp(buffer: &[u8], width: i32, height: i32) -> anyhow::Result<SoftwareBitmap> {
    let bmp = SoftwareBitmap::Create(BitmapPixelFormat::Rgba8, width, height)?;
    {
        let bmp_buf = bmp.LockBuffer(BitmapBufferAccessMode::ReadWrite)?;
        let array: IMemoryBufferByteAccess = bmp_buf.CreateReference()?.cast()?;

        let mut data = std::ptr::null_mut();
        let mut capacity = 0;
        unsafe {
            array.GetBuffer(&mut data, &mut capacity)?;
        }
        assert_eq!((width * height * 4).abs(), capacity as i32);

        let slice = unsafe { std::slice::from_raw_parts_mut(data, capacity as usize) };
        slice.chunks_mut(4).enumerate().for_each(|(i, c)| {
            c[0] = buffer[3 * i];
            c[1] = buffer[3 * i + 1];
            c[2] = buffer[3 * i + 2];
            c[3] = 255;
        });
    }

    Ok(bmp)
}

async fn words_from_image_buffer(
    buffer: &[u8],
    width: i32,
    height: i32,
) -> anyhow::Result<Vec<Word>> {
    let bmp = make_bmp(buffer, width, height)?;
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(&bmp)?.await?.Lines()?;
    let mut collected_words: Vec<Word> = Vec::new();

    result.into_iter().for_each(|line| {
        let words = line.Words().unwrap();
        let line_text = line.Text().unwrap().to_string_lossy();
        let mut _x = 0.0f64;
        let mut _y = 0.0f64;
        let mut line_heigth = 0.0;
        let mut line_width = 0.0;
        let mut idx = 0;
        words.into_iter().for_each(|word| {
            let rect = word.BoundingRect().unwrap();
            let name = &word.Text().unwrap().to_string_lossy();
            collected_words.push(Word {
                x: rect.X.into(),
                y: rect.Y.into(),
                text: name.to_string(),
                height: rect.Height.into(),
                width: rect.Width.into(),
            });
            if idx == 0 {
                _x = rect.X as f64;
            }
            if line_heigth < rect.Height as f64 {
                line_heigth = rect.Height as f64;
            }
            line_width += rect.Width as f64;
            if _y < rect.Y as f64 {
                _y = rect.Y as f64;
            }
            idx += 1;
        });
        collected_words.push(Word {
            x: _x,
            y: _y,
            text: line_text.replace(" ", ""),
            height: line_heigth,
            width: line_width,
        })
    });
    Ok(collected_words)
}
