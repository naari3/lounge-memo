use async_trait::async_trait;
use image::{ImageBuffer, Luma, Pixel, Rgb};
use template_matching::{find_extremes, MatchTemplateMethod, TemplateMatcher};

use crate::{
    detector::{CourseDetector, PositionDetector},
    mogi_result::MogiResult,
};

use super::Detector;

// based 1280 x 720
const FLAG_CHECK_PATTERN: [(u32, u32); 9] = [
    (174, 659),
    (183, 659),
    (192, 659),
    (174, 667),
    (180, 667),
    (189, 667),
    (173, 675),
    (182, 675),
    (191, 675),
];

pub struct RaceFinishDetector {
    race_kind: RaceKind,
    results_image: ImageBuffer<Luma<f32>, Vec<f32>>,
    results_mask_image: ImageBuffer<Luma<f32>, Vec<f32>>,
    results_matcher: TemplateMatcher,
    on_results_vec: Vec<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaceKind {
    Internet,
    #[allow(dead_code)]
    Local,
}

impl RaceFinishDetector {
    pub fn new() -> RaceFinishDetector {
        println!("RaceFinishDetector");
        RaceFinishDetector {
            race_kind: RaceKind::Internet,
            results_image: image::load_from_memory(include_bytes!("../assets/results.png"))
                .unwrap()
                .to_luma32f(),
            results_mask_image: image::load_from_memory(include_bytes!(
                "../assets/results_mask.png"
            ))
            .unwrap()
            .to_luma32f(),
            results_matcher: TemplateMatcher::new(),
            on_results_vec: Vec::new(),
        }
    }

    fn eval_on_result_with_match_template(&mut self, input: &ImageBuffer<Luma<f32>, Vec<f32>>) {
        self.results_matcher.match_template_mask(
            input,
            &self.results_image,
            &self.results_mask_image,
            MatchTemplateMethod::SumOfSquaredDifferences,
        );
        let results = self.results_matcher.wait_for_result();
        let location_offset_x_min: u32 = match self.race_kind {
            RaceKind::Internet => 555,
            RaceKind::Local => 595,
        };
        // let location_offset_x_max = 605;
        let location_offset_x_max = match self.race_kind {
            RaceKind::Internet => 568,
            RaceKind::Local => 605,
        };
        if let Some(results) = results {
            let extremes = find_extremes(&results);
            println!("results: {:?}", extremes.max_value_location);
            if extremes.max_value_location.0 >= location_offset_x_min
                && extremes.max_value_location.0 <= location_offset_x_max
            {
                if extremes.max_value_location.1 >= 42 && extremes.max_value_location.1 <= 57 {
                    self.on_results_vec.push(true);
                    if self.on_results_vec.len() > 4 {
                        self.on_results_vec.remove(0);
                        return;
                    }
                }
            }
        }
        self.on_results_vec.push(false);
        if self.on_results_vec.len() > 4 {
            self.on_results_vec.remove(0);
        }
    }

    fn is_on_result(&self) -> bool {
        // もし配列の中にtrueが3つ以上あれば、レース結果画面にいると判断する
        self.on_results_vec.iter().filter(|b| **b).count() >= 3
    }
}

#[async_trait]
impl Detector for RaceFinishDetector {
    async fn detect(
        mut self: Box<Self>,
        buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        mogi_result: &mut MogiResult,
    ) -> anyhow::Result<Box<dyn Detector + Send + Sync>> {
        if self.detect_error(buffer, mogi_result).await? {
            return Ok(Box::new(CourseDetector::new()));
        }

        for (i, (x, y)) in FLAG_CHECK_PATTERN.into_iter().enumerate() {
            let pixel = buffer.get_pixel(x, y);
            let channels = pixel.channels();
            let r = channels[0];
            let g = channels[1];
            let b = channels[2];
            if (i % 2) == 0 {
                if r < 5 && g < 5 && b < 5 {
                    println!("flag is on view");
                    return Ok(self);
                }
            } else {
                if r > 0xD0 && g > 0xD0 && b > 0xD0 {
                    println!("flag is on view");
                    return Ok(self);
                }
            }
        }

        let input = image::DynamicImage::ImageRgb8(buffer.clone());
        let input = input.to_luma32f();
        self.eval_on_result_with_match_template(&input);
        if self.is_on_result() {
            return Ok(Box::new(PositionDetector::new()));
        }
        Ok(self)
    }
}
