use crate::detector::RaceFinishDetector;
use crate::{courses::get_course_by_words, mogi_result::MogiResult, word::Word, WIDTH};
use async_trait::async_trait;
use image::ImageBuffer;
use image::Rgb;

use super::{words_from_image_buffer, Detector};

pub struct CourseDetector;

#[async_trait]
impl Detector for CourseDetector {
    async fn detect(
        self: Box<Self>,
        buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        mogi_result: &mut MogiResult,
    ) -> anyhow::Result<Box<dyn Detector + Send + Sync>> {
        println!("CourseDetector");
        let words = match words_from_image_buffer(
            buffer,
            buffer.width().try_into()?,
            buffer.height().try_into()?,
        )
        .await
        {
            Ok(w) => w,
            Err(e) => {
                println!("Error: {:?}", e);
                return Ok(self);
            }
        };
        // println!("words: {:?}", &words);
        let for_course_texts = words
            .into_iter()
            .filter(filter_for_course_texts)
            .collect::<Vec<Word>>();

        if for_course_texts.len() > 0 {
            println!("for_course_texts: {:?}", &for_course_texts);
        }

        let course = get_course_by_words(&for_course_texts);
        if let Some(course) = course {
            println!("course: {:?}", &course);
            mogi_result.set_current_course(course);
            return Ok(Box::new(RaceFinishDetector::new()));
        }

        Ok(self)
    }
}

fn filter_for_course_texts(word: &Word) -> bool {
    // コース名は画面下部にあるので、上部の文字は除外
    let lower: f64 = 950.0 / 1920.0 * (WIDTH as f64);
    if word.y < lower {
        return false;
    }

    // 7文字以上の場合、コース名っぽいので通す
    if word.text.chars().count() > 6 {
        return true;
    }

    let lower_text = word.text.to_lowercase();

    // SFC, GBA, n64, GC, DS, Wii, 3DS が入っていたら通す
    let is_series_text = lower_text.contains("sfc")
        || lower_text.contains("gba")
        || lower_text.contains("n64")
        || lower_text.contains("gc")
        || lower_text.contains("ds")
        || lower_text.contains("wii")
        || lower_text.contains("3ds")
        || lower_text.contains("tour");
    return is_series_text;
}
