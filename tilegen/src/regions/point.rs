#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TilePoint {
    pub x: usize,
    pub y: usize,
}

#[allow(unused)]
impl TilePoint {
    pub fn new(x: usize, y: usize) -> TilePoint {
        TilePoint { x, y }
    }

    /// Calculate the distance between this and another point
    pub fn distance(&self, other: &TilePoint) -> f32 {
        let a = (other.x as i32 - self.x as i32).pow(2).abs();
        let b = (other.y as i32 - self.y as i32).pow(2).abs();
        (a as f32 + b as f32).sqrt()
    }
}
