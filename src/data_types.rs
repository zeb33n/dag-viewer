// rgba
pub type Colour = u32;

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

pub type LogicalNodeHandle = usize;
