use std::ops::{self, AddAssign, SubAssign};
// rgba
pub type Colour = u32;


#[derive(Clone)]
pub struct VecF2 {
    pub x: f32,
    pub y: f32,
}

pub struct VecI2 {
    pub x: i32,
    pub y: i32,
}

impl VecF2 {
    pub fn from_veci2(vec: VecI2) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
        }
    }

    pub fn as_veci2(&self) -> VecI2 {
        VecI2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl ops::Add<VecF2> for VecF2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for VecF2 {
    fn add_assign(&mut self, rhs: VecF2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<VecF2> for VecF2 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for VecF2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f32> for VecF2 {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::MulAssign<f32> for VecF2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl VecF2 {
    pub fn dot(&self, rhs: &VecF2) -> f32 {
        self.x * rhs.x + self.y * rhs.y 
    }
}


impl VecI2 {
    pub fn from_vecf2(vec: VecF2) -> Self {
        Self {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }

    pub fn as_vecf2(&self) -> VecF2 {
        VecF2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}


pub struct Circle {
    pub center: VecF2,
    pub radius: f32,
}

pub type LogicalNodeHandle = usize;
