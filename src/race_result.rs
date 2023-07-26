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

    pub fn to_score(&self) -> u32 {
        self.position.to_score()
    }
}

impl Display for RaceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(course) = &self.course {
            write!(f, "{}\t", course)?;
        }
        write!(f, "{}\t", self.position)?;
        write!(f, "{}", self.to_score())
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

    pub fn to_score(self) -> u32 {
        match self {
            Position::First => 15,
            Position::Second => 12,
            Position::Third => 10,
            Position::Fourth => 9,
            Position::Fifth => 8,
            Position::Sixth => 7,
            Position::Seventh => 6,
            Position::Eighth => 5,
            Position::Ninth => 4,
            Position::Tenth => 3,
            Position::Eleventh => 2,
            Position::Twelfth => 1,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Position::First => "1",
            Position::Second => "2",
            Position::Third => "3",
            Position::Fourth => "4",
            Position::Fifth => "5",
            Position::Sixth => "6",
            Position::Seventh => "7",
            Position::Eighth => "8",
            Position::Ninth => "9",
            Position::Tenth => "10",
            Position::Eleventh => "11",
            Position::Twelfth => "12",
        };
        write!(f, "{}", text)
    }
}
