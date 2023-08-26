use std::fmt::Display;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::{
    courses::Course,
    race_result::{Position, RaceResult},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MogiResult {
    races: Vec<RaceResult>,
    current_course: Option<Course>,
    created_at: chrono::DateTime<chrono::Local>,
}

impl MogiResult {
    pub fn new() -> MogiResult {
        MogiResult {
            races: Vec::new(),
            current_course: None,
            created_at: chrono::Local::now(),
        }
    }

    pub fn set_current_course(&mut self, course: Course) {
        self.current_course = Some(course);
    }

    pub fn set_current_position(&mut self, position: Position) {
        let current_course = self.current_course.clone();
        if let Some(current_course) = current_course {
            let race = RaceResult::new(Some(current_course), position);
            self.races.push(race);
            self.current_course = None;
        }
    }

    pub fn reset_current_course(&mut self) {
        self.current_course = None;
    }

    pub fn iter_races(&self) -> std::slice::Iter<RaceResult> {
        self.races.iter()
    }

    pub fn current_course(&self) -> &Option<Course> {
        &self.current_course
    }

    pub fn set_course(&mut self, index: usize, course: Course) {
        self.races[index].set_course(course);
    }

    pub fn set_position(&mut self, index: usize, position: Position) {
        self.races[index].set_position(position);
    }

    pub fn total_score(&self) -> u32 {
        self.races.iter().map(|r| r.to_score()).sum::<u32>()
    }

    pub fn save_result_image(&self, buffer: &image::RgbImage, prefix: &str) -> anyhow::Result<()> {
        let path = format!(
            "results/{}/{}_{:02}.png",
            self.created_at.format("%Y%m%d-%H%M%S"),
            prefix,
            self.races.len(),
        );
        // ディレクトリがなければ作る
        std::fs::create_dir_all(format!(
            "results/{}",
            self.created_at.format("%Y%m%d-%H%M%S")
        ))?;
        buffer.save(path)?;
        Ok(())
    }

    pub fn save_result(&self) -> anyhow::Result<()> {
        let path = format!(
            "results/{}/result.txt",
            self.created_at.format("%Y%m%d-%H%M%S")
        );
        // ディレクトリがなければ作る
        std::fs::create_dir_all(format!(
            "results/{}",
            self.created_at.format("%Y%m%d-%H%M%S")
        ))?;
        let mut file = std::fs::File::create(path)?;
        write!(file, "{}", self)?;
        Ok(())
    }
}

impl Display for MogiResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current_course = self.current_course.clone();
        for (i, race) in self.races.iter().enumerate() {
            writeln!(f, "{:02}\t{race}", i + 1)?;
        }
        writeln!(f, "---")?;
        if let Some(current_course) = current_course {
            writeln!(f, "current course: {}", current_course)?;
        }
        let total_score = self.total_score();
        writeln!(f, "total score: {total_score}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::courses::Console;

    use super::*;

    #[test]
    fn test_mogi_result() {
        let mut mogi_result = MogiResult::new();
        mogi_result.set_current_course(Course::new("ドルフィンみさき".to_string(), Console::New));
        mogi_result.set_current_position(Position::First);
        assert_eq!(mogi_result.races.len(), 1);

        mogi_result.set_current_course(Course::new("ヨッシーアイランド".to_string(), Console::New));
        mogi_result.set_current_position(Position::Second);
        assert_eq!(mogi_result.total_score(), 27);
    }

    #[test]
    fn test_mogi_result_reset_current_course() {
        let mut mogi_result = MogiResult::new();
        mogi_result.set_current_course(Course::new("ドルフィンみさき".to_string(), Console::New));
        mogi_result.reset_current_course();
        assert_eq!(mogi_result.current_course, None);
    }
}
