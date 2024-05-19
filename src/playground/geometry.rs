
#[derive(Debug)]
pub struct Rectangle {
    pub base: u32,
    pub height: u32
}

impl Rectangle {

    #[allow(dead_code)]
    pub fn area(&self) -> u32 {
        self.base * self.height
    }
    #[allow(dead_code)]
    pub fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.base > rect.base
    }

    #[allow(dead_code)]
    pub fn square(size: u32) -> Self {
        Self {
            base: size,
            height: size,
        }
    }
}