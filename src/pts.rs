use std::ops::*;

macro_rules! sqr {
    ($e:expr) => {
        $e * $e
    };
}

#[derive(Debug, Copy, Clone)]
pub struct Pt2 {
    pub x: f32,
    pub y: f32,
}
impl Pt2 {
    pub const NULL: Self = Pt2 { x: 0., y: 0. };
    pub fn new(x: f32, y: f32) -> Self {
        Pt2 { x, y }
    }
    pub fn length(self) -> f32 {
        (sqr!(self.x) + sqr!(self.y)).sqrt()
    }
    pub fn with_length(self, new_length: f32) -> Self {
        let l = self.length();
        let m = new_length / l;
        self * m
    }
}

impl Add for Pt2 {
    type Output = Pt2;
    fn add(self, other: Pt2) -> Pt2 {
        Pt2::new(self.x + other.x, self.y + other.y)
    }
}
impl<'a> Add for &'a Pt2 {
    type Output = Pt2;
    fn add(self, other: &Pt2) -> Pt2 {
        Pt2::new(self.x + other.x, self.y + other.y)
    }
}
impl Sub for Pt2 {
    type Output = Pt2;
    fn sub(self, other: Pt2) -> Pt2 {
        Pt2::new(self.x - other.x, self.y - other.y)
    }
}
impl<'a> Sub for &'a Pt2 {
    type Output = Pt2;
    fn sub(self, other: &Pt2) -> Pt2 {
        Pt2::new(self.x - other.x, self.y - other.y)
    }
}
impl AddAssign for Pt2 {
    fn add_assign(&mut self, other: Pt2) {
        *self = *self + other;
    }
}
impl SubAssign for Pt2 {
    fn sub_assign(&mut self, other: Pt2) {
        *self = *self - other;
    }
}
impl Not for Pt2 {
    type Output = Pt2;
    fn not(self) -> Pt2 {
        Pt2::new(-self.x, -self.y)
    }
}
impl Mul<f32> for Pt2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Pt2::new(self.x * scalar, self.y * scalar)
    }
}
