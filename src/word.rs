#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
}

impl Word {
    pub fn new(text: String, x: f64, y: f64, height: f64, width: f64) -> Word {
        Word {
            text,
            x,
            y,
            height,
            width,
        }
    }
}
