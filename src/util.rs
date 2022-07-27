#[derive(PartialEq)]
pub enum State {
    Playing,
    Paused,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub type Vector2 = Point;
