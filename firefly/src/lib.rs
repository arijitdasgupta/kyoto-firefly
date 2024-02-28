use game_container::game_container::{GameContainer, Keys, Pixel, Pixels};
use rand::Rng;
pub struct Game {
    width: u16,
    height: u16,
    framebuffer: Pixels
}

impl Game {
    fn new(width: u16, height: u16) -> Self {
        let mut fb = Vec::new();

        for i in 0..(width * height) {
            fb.push(Pixel { r: 0, g: 0, b: 0, a: 0 })
        }

        Self { width, height, framebuffer: fb }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

impl GameContainer for Game {
    fn tick(self: Self, _keys: &Vec<Keys>) {
        println!("key pressed");

        for i in 0..(self.width * self.height) {
            self.framebuffer[i] = Pixel { r: rng::thread }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
