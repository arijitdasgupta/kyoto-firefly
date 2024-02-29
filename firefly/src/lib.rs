use game_container::game_container::{GameContainer, Keys, Pixel, Pixels};
pub struct Game {
    pub width: u16,
    pub height: u16,
    pub framebuffer: Pixels
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        let mut fb = Vec::new();
        let max_index = width as usize * height as usize;

        for _ in 0..max_index {
            fb.push(Pixel { r: 0, g: 0, b: 0, a: 0 })
        }

        Self { width, height, framebuffer: fb }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

impl GameContainer for Game {
    fn tick(self: &mut Self, _keys: &Vec<Keys>) {
        let max_idx = self.width as usize * self.height as usize;

        for i in 0..max_idx {
            self.framebuffer[i as usize] = Pixel { r: 100, g: 0, b: 0, a: 255 }
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
