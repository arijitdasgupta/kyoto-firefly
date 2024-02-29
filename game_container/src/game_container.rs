pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub type Pixels = Vec<Pixel>;

pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select
}

pub trait GameContainer {
    fn tick(self: &mut Self, keys: &Vec<Keys>);
}