use crate::courses::get_course_by_words_with_nearest;
use crate::detector::RaceFinishDetector;
use crate::HEIGHT;
use crate::{courses::get_course_by_words, mogi_result::MogiResult, word::Word, WIDTH};
use async_trait::async_trait;
use image::Rgb;
use image::{ImageBuffer, Luma};

use super::{words_from_image_buffer, Detector};

pub struct CourseDetector {
    on_results_vec: Vec<bool>,
}

impl CourseDetector {
    pub fn new() -> CourseDetector {
        log::debug!("CourseDetector");
        CourseDetector {
            on_results_vec: Vec::new(),
        }
    }

    fn eval_on_course_wait_room(&mut self, input: &ImageBuffer<Luma<f32>, Vec<f32>>) {
        let result = input.pixels().take(WIDTH * 50).all(|p| p.0[0] < 0.1);
        if result {
            self.on_results_vec.push(true);
            if self.on_results_vec.len() > 5 {
                self.on_results_vec.remove(0);
                return;
            }
        }
        self.on_results_vec.push(false);
        if self.on_results_vec.len() > 5 {
            self.on_results_vec.remove(0);
        }
    }

    fn is_on_course_wait_room(&self) -> bool {
        self.on_results_vec.iter().all(|b| *b)
    }
}

#[async_trait]
impl Detector for CourseDetector {
    async fn detect(
        mut self: Box<Self>,
        buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        mogi_result: &mut MogiResult,
    ) -> anyhow::Result<Box<dyn Detector + Send + Sync>> {
        // TODO: まだ実行していないので明日実行してみる
        let input = image::DynamicImage::ImageRgb8(buffer.clone());
        let input = input.to_luma32f();
        self.eval_on_course_wait_room(&input);
        if !self.is_on_course_wait_room() {
            return Ok(self);
        }

        let words = match words_from_image_buffer(
            buffer,
            buffer.width().try_into()?,
            buffer.height().try_into()?,
        )
        .await
        {
            Ok(w) => w,
            Err(e) => {
                log::error!("Error: {:?}", e);
                return Ok(self);
            }
        };
        if words.len() > 0 {
            log::trace!("words: {:?}", &words);
        }
        let for_course_texts = words
            .into_iter()
            .filter(filter_for_course_texts)
            .collect::<Vec<Word>>();

        if for_course_texts.len() > 0 {
            log::trace!("for_course_texts: {:?}", &for_course_texts);
        }

        let course = get_course_by_words(&for_course_texts);
        if let Some(course) = course {
            log::debug!("course: {:?}", &course);
            mogi_result.set_current_course(course);
            return Ok(Box::new(RaceFinishDetector::new()));
        } else {
            let course = get_course_by_words_with_nearest(&for_course_texts, 4);
            if let Some(course) = course {
                log::debug!("course with nearest: {:?}", &course);
                mogi_result.set_current_course(course);
                return Ok(Box::new(RaceFinishDetector::new()));
            }
        }

        Ok(self)
    }
}

fn filter_for_course_texts(word: &Word) -> bool {
    // コース名は画面下部にあるので、上部の文字は除外
    let lower: f64 = 950.0 / 1080.0 * (HEIGHT as f64);
    if word.y < lower {
        return false;
    }

    // 6文字以上の場合、コース名っぽいので通す
    if word.text.chars().count() >= 6 {
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
