use std::fmt::Display;

use crate::courses::Course;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaceResult {
    course: Option<Course>,
    position: Position,
}

impl RaceResult {
    pub fn new(course: Option<Course>, position: Position) -> RaceResult {
        RaceResult { course, position }
    }
}

impl Display for RaceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(course) = &self.course {
            write!(f, "{}\t", course)?;
        }
        write!(f, "{}", self.position)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
}

impl Position {
    pub fn from_index(index: usize) -> Position {
        match index {
            0 => Position::First,
            1 => Position::Second,
            2 => Position::Third,
            3 => Position::Fourth,
            4 => Position::Fifth,
            5 => Position::Sixth,
            6 => Position::Seventh,
            7 => Position::Eighth,
            8 => Position::Ninth,
            9 => Position::Tenth,
            10 => Position::Eleventh,
            11 => Position::Twelfth,
            _ => panic!("invalid index: {}", index),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Position::First => "1st",
            Position::Second => "2nd",
            Position::Third => "3rd",
            Position::Fourth => "4th",
            Position::Fifth => "5th",
            Position::Sixth => "6th",
            Position::Seventh => "7th",
            Position::Eighth => "8th",
            Position::Ninth => "9th",
            Position::Tenth => "10th",
            Position::Eleventh => "11th",
            Position::Twelfth => "12th",
        };
        write!(f, "{}", text)
    }
}
