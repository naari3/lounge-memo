use std::fmt::Display;

use crate::{
    courses::Course,
    race_result::{Position, RaceResult},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MogiResult {
    races: Vec<RaceResult>,
    current_course: Option<Course>,
}

impl MogiResult {
    pub fn new() -> MogiResult {
        MogiResult {
            races: Vec::new(),
            current_course: None,
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

    pub fn total_score(&self) -> u32 {
        self.races.iter().map(|r| r.to_score()).sum::<u32>()
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
    use crate::courses::Series;

    use super::*;

    #[test]
    fn test_mogi_result() {
        let mut mogi_result = MogiResult::new();
        mogi_result.set_current_course(Course::new("ドルフィンみさき".to_string(), Series::New));
        mogi_result.set_current_position(Position::First);
        assert_eq!(mogi_result.races.len(), 1);

        mogi_result.set_current_course(Course::new("ヨッシーアイランド".to_string(), Series::New));
        mogi_result.set_current_position(Position::Second);
        assert_eq!(mogi_result.total_score(), 27);
    }

    #[test]
    fn test_mogi_result_reset_current_course() {
        let mut mogi_result = MogiResult::new();
        mogi_result.set_current_course(Course::new("ドルフィンみさき".to_string(), Series::New));
        mogi_result.reset_current_course();
        assert_eq!(mogi_result.current_course, None);
    }
}
