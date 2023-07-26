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
        let race = RaceResult::new(current_course, position);
        println!("race: {:?}", race);
        self.races.push(race);
        self.current_course = None;
    }
}

impl Display for MogiResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for race in &self.races {
            writeln!(f, "{}", race)?;
        }
        Ok(())
    }
}
